#!/usr/bin/env bash
set -euo pipefail

# Usage: check-workspace-versions.sh [ffi_lib1 ffi_lib2 ...]
# If arguments are given, checks those FFI crates and their corresponding lib crates.
# FFI lib names use underscores (e.g. "core trig linear_algebra").
# If no arguments are given, all crates under crates/lib/ and crates/ffi/ are checked.
# Exits non-zero if any workspace dependency version in the root Cargo.toml
# does not match the crate's own [package].version.

if [ $# -gt 0 ]; then
  tomls=()
  for mod in "$@"; do
    crate_dir=$(echo "$mod" | tr '_' '-')
    tomls+=("crates/lib/${crate_dir}/Cargo.toml")
    tomls+=("crates/ffi/${crate_dir}/Cargo.toml")
  done
else
  tomls=(crates/lib/*/Cargo.toml crates/ffi/*/Cargo.toml)
fi

failed=false

echo "::group::Check workspace dependency versions"
for toml in "${tomls[@]}"; do
  [ -f "$toml" ] || continue
  crate_path=$(dirname "$toml")
  crate_ver=$(grep '^version' "$toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
  ws_line=$(grep "path = \"${crate_path}\"" Cargo.toml) || continue
  ws_ver=$(echo "$ws_line" | sed 's/.*version = "\([^"]*\)".*/\1/')

  if [ "$crate_ver" != "$ws_ver" ]; then
    echo "::error file=Cargo.toml::Workspace dep version ${ws_ver} for ${crate_path} does not match crate version ${crate_ver}"
    failed=true
  else
    echo "  ${crate_path} v${crate_ver} matches workspace dep"
  fi
done
echo "::endgroup::"

if $failed; then
  echo "::error::Workspace dependency versions must match crate versions — run ./scripts/sync-workspace-versions.sh to fix"
  exit 1
fi
