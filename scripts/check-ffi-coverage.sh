#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

latest_tag_for() {
  git -C "$REPO_ROOT" tag --list "$1/v*" --sort=-v:refname | head -1
}

version_from_tag() {
  echo "${1##*/v}"
}

detect_bump() {
  local prev="$1" cur="$2"
  IFS='.' read -r p_major p_minor _p_patch <<< "$prev"
  IFS='.' read -r c_major c_minor _c_patch <<< "$cur"
  if [ "$c_major" -ne "$p_major" ]; then
    echo "major"
  elif [ "$c_minor" -ne "$p_minor" ]; then
    echo "minor"
  else
    echo "patch"
  fi
}

if [ "${1:-}" = "--cached" ]; then
  changed_files=$(git diff --cached --name-only)
else
  base_ref="${1:-origin/main}"
  head_ref="${2:-HEAD}"
  changed_files=$(git diff --name-only "$base_ref"..."$head_ref")
fi

missing=()
missing_ffi_src=()

for file in $changed_files; do
  case "$file" in
    crates/ffi/*) continue ;;
    crates/lib/*/src/*)
      crate_name=$(echo "$file" | sed 's|crates/lib/\([^/]*\)/.*|\1|')
      ffi_dir="crates/ffi/$crate_name"
      [ -d "$ffi_dir" ] || continue

      if ! echo "$changed_files" | grep -q "^crates/ffi/${crate_name}/"; then
        missing+=("$crate_name")
      fi
      ;;
  esac
done

# Deduplicate
if [ ${#missing[@]} -gt 0 ]; then
  mapfile -t missing < <(printf '%s\n' "${missing[@]}" | sort -u)
fi

# For lib crates with a minor or major version bump, require .rs changes in FFI src
for lib_toml in crates/lib/*/Cargo.toml; do
  [ -f "$lib_toml" ] || continue
  crate_name=$(basename "$(dirname "$lib_toml")")
  [ -d "crates/ffi/$crate_name" ] || continue

  lib_tag=$(latest_tag_for "crates/${crate_name}")
  [ -z "$lib_tag" ] && continue

  prev_version=$(version_from_tag "$lib_tag")
  cur_version=$(grep '^version' "$lib_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
  [ "$prev_version" = "$cur_version" ] && continue

  bump=$(detect_bump "$prev_version" "$cur_version")
  [ "$bump" = "patch" ] && continue

  if ! echo "$changed_files" | grep -q "^crates/ffi/${crate_name}/src/.*\.rs$"; then
    missing_ffi_src+=("$crate_name ($bump bump: $prev_version -> $cur_version)")
  fi
done

failed=false

if [ ${#missing[@]} -gt 0 ]; then
  echo "::warning::Library crates changed without corresponding FFI crate updates:"
  for name in "${missing[@]}"; do
    echo "  - crates/lib/$name/src/ changed but crates/ffi/$name/ was not modified"
  done
  failed=true
fi

if [ ${#missing_ffi_src[@]} -gt 0 ]; then
  echo "::error::Library crates have minor/major version bumps without FFI source changes:"
  for entry in "${missing_ffi_src[@]}"; do
    echo "  - $entry — expected .rs changes in crates/ffi/$(echo "$entry" | cut -d' ' -f1)/src/"
  done
  failed=true
fi

if $failed; then
  exit 1
fi
