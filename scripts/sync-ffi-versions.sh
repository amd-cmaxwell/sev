#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRATES_DIR="$REPO_ROOT/crates"
FFI_DIR="$CRATES_DIR/ffi"

latest_tag_for() {
  git -C "$REPO_ROOT" tag --list "$1/v*" --sort=-v:refname | head -1
}

version_from_tag() {
  echo "${1##*/v}"
}

version_lt() {
  IFS='.' read -r a_major a_minor a_patch <<< "$1"
  IFS='.' read -r b_major b_minor b_patch <<< "$2"
  if [ "$a_major" -lt "$b_major" ]; then return 0; fi
  if [ "$a_major" -eq "$b_major" ] && [ "$a_minor" -lt "$b_minor" ]; then return 0; fi
  if [ "$a_major" -eq "$b_major" ] && [ "$a_minor" -eq "$b_minor" ] && [ "$a_patch" -lt "$b_patch" ]; then return 0; fi
  return 1
}

apply_bump() {
  local version="$1" bump="$2"
  IFS='.' read -r major minor patch <<< "$version"
  case "$bump" in
    major) echo "$((major + 1)).0.0" ;;
    minor) echo "${major}.$((minor + 1)).0" ;;
    patch) echo "${major}.${minor}.$((patch + 1))" ;;
  esac
}

compute_ffi_target() {
  local ffi_version="$1" lib_version="$2"
  if version_lt "$ffi_version" "$lib_version"; then
    echo "$lib_version"
  else
    apply_bump "$ffi_version" "patch"
  fi
}

changed=()

for lib_toml in "$CRATES_DIR"/lib/*/Cargo.toml; do
  crate_dir="$(basename "$(dirname "$lib_toml")")"

  ffi_toml="$FFI_DIR/$crate_dir/Cargo.toml"
  [ -f "$ffi_toml" ] || continue

  lib_version="$(grep '^version' "$lib_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')"
  ffi_version="$(grep '^version' "$ffi_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')"

  lib_tag=$(latest_tag_for "crates/${crate_dir}")

  if [ -z "$lib_tag" ]; then
    if [ "$lib_version" != "$ffi_version" ]; then
      new_ffi_version="$lib_version"
      echo >&2 "  synced $crate_dir (new crate): $ffi_version -> $new_ffi_version"
    else
      continue
    fi
  else
    prev_lib_version=$(version_from_tag "$lib_tag")

    if [ "$prev_lib_version" = "$lib_version" ]; then
      continue
    fi

    # Idempotency: if the FFI version already differs from HEAD, this sync
    # (or a manual bump) already ran in this working tree — skip.
    head_ffi_version=$(git -C "$REPO_ROOT" show HEAD:"crates/ffi/${crate_dir}/Cargo.toml" 2>/dev/null \
      | grep '^version' | head -1 | sed 's/.*"\(.*\)"/\1/') || true
    if [ -n "$head_ffi_version" ] && [ "$ffi_version" != "$head_ffi_version" ]; then
      echo >&2 "  $crate_dir: FFI already bumped in working tree ($head_ffi_version -> $ffi_version) — skipping"
      continue
    fi

    new_ffi_version=$(compute_ffi_target "$ffi_version" "$lib_version")

    echo >&2 "  synced $crate_dir: $ffi_version -> $new_ffi_version (lib changed: $prev_lib_version -> $lib_version)"
  fi

  ffi_path="crates/ffi/${crate_dir}"
  sed "s/^version = \"${ffi_version}\"/version = \"${new_ffi_version}\"/" "$ffi_toml" > "${ffi_toml}.tmp" && mv "${ffi_toml}.tmp" "$ffi_toml"
  sed "s|\(path = \"${ffi_path}\", version = \)\"${ffi_version}\"|\1\"${new_ffi_version}\"|" "$REPO_ROOT/Cargo.toml" > "$REPO_ROOT/Cargo.toml.tmp" && mv "$REPO_ROOT/Cargo.toml.tmp" "$REPO_ROOT/Cargo.toml"
  crate_us="$(echo "$crate_dir" | tr '-' '_')"
  changed+=("$crate_us")
done

if [ ${#changed[@]} -gt 0 ]; then
  echo "${changed[*]}"
fi
