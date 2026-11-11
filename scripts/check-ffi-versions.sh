#!/usr/bin/env bash
set -euo pipefail

# Usage: check-ffi-versions.sh [ffi_lib1 ffi_lib2 ...]
# If no arguments are given, all crates under crates/ffi/ are checked.
# FFI lib names use underscores (e.g. "core trig linear_algebra").

echo "::group::FFI version alignment check"
failed=false
checked=0
skipped=0

if [ $# -gt 0 ]; then
  tomls=()
  for mod in "$@"; do
    crate_dir=$(echo "$mod" | tr '_' '-')
    tomls+=("crates/ffi/${crate_dir}/Cargo.toml")
  done
else
  tomls=(crates/ffi/*/Cargo.toml)
fi

for ffi_toml in "${tomls[@]}"; do
  [ -f "$ffi_toml" ] || continue
  crate_dir=$(basename "$(dirname "$ffi_toml")")
  lib_toml="crates/lib/${crate_dir}/Cargo.toml"
  if [ ! -f "$lib_toml" ]; then
    echo "::notice::Skipping crates/ffi/${crate_dir} — no corresponding lib crate"
    skipped=$((skipped + 1))
    continue
  fi
  ffi_ver=$(grep '^version' "$ffi_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
  lib_ver=$(grep '^version' "$lib_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
  IFS='.' read -r ffi_major ffi_minor ffi_patch <<< "$ffi_ver"
  IFS='.' read -r lib_major lib_minor lib_patch <<< "$lib_ver"
  if [ "$ffi_major" -lt "$lib_major" ] ||
     { [ "$ffi_major" -eq "$lib_major" ] && [ "$ffi_minor" -lt "$lib_minor" ]; } ||
     { [ "$ffi_major" -eq "$lib_major" ] && [ "$ffi_minor" -eq "$lib_minor" ] && [ "$ffi_patch" -lt "$lib_patch" ]; }; then
    echo "::error file=${ffi_toml}::crates/ffi/${crate_dir} v${ffi_ver} is behind crates/lib/${crate_dir} v${lib_ver} — FFI version must be >= lib version"
    failed=true
  else
    echo "  crates/ffi/${crate_dir} v${ffi_ver} >= crates/lib/${crate_dir} v${lib_ver}"
  fi
  checked=$((checked + 1))
done
echo "::endgroup::"
echo "::notice::Checked ${checked} FFI/lib pairs, skipped ${skipped} FFI-only crates"
if $failed; then
  echo "::error::One or more FFI crate versions are behind their lib crates"
  exit 1
fi
