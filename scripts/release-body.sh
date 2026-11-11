#!/usr/bin/env bash
set -euo pipefail

# Usage: release-body.sh <tag> <asset1> [asset2 ...]
# Generates a markdown release body with a platform table for the given assets.
# Output is written to stdout.

tag="$1"
shift
assets=("$@")

repo_url="${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}"

if [ ${#assets[@]} -eq 0 ] && [ -z "${BINDINGS_COMMIT:-}" ]; then
  exit 0
fi

is_ffi=false
case "$tag" in
  crates/ffi/*)
    is_ffi=true
    lib_crate="${tag#crates/ffi/}" lib_crate="${lib_crate%%/*}"
    if [ -f "crates/lib/${lib_crate}/Cargo.toml" ]; then
      lib_ver=$(grep '^version' "crates/lib/${lib_crate}/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
      lib_tag="crates/${lib_crate}/v${lib_ver}"
      lib_release_link="[${REPO_NAME:-}-${lib_crate} v${lib_ver}](${repo_url}/releases/tag/${lib_tag})"
      echo "> **FFI-only release** — this updates the foreign function interface bindings for \`${lib_crate}\`, not the core Rust library. The underlying lib crate release is ${lib_release_link}."
    else
      echo "> **FFI-only release** — this updates the \`${lib_crate}\` FFI crate. There is no corresponding core Rust library crate."
    fi
    echo
    ;;
esac

# Extract crate name and version from the tag (crates/{name}/vX.Y.Z or crates/ffi/{name}/vX.Y.Z)
version=$(echo "$tag" | grep -o 'v[0-9].*')
crate_dir=$(echo "$tag" | sed "s|/${version}$||" | xargs basename)

# Build bindings source links using the ffi/ and bindings/go/ tags
repo_us=$(echo "${REPO_NAME:-}" | tr '-' '_')
suffix_us=$(echo "${PKG_SUFFIX:-}" | tr '-' '_')
crate_us=$(echo "$crate_dir" | tr '-' '_')
ffi_tag="ffi/${crate_dir}/${version}"
go_pkg="${repo_us}_${crate_us}${suffix_us}"
go_tag="bindings/go/${go_pkg}/${version}"

py_pkg="${REPO_NAME}-${crate_dir}${PKG_SUFFIX:-}"
npm_scope="${NPM_SCOPE:-@${REPO_OWNER:-}}"
js_pkg="${npm_scope}/${REPO_NAME}-${crate_dir}${PKG_SUFFIX:-}"

ffi_crate_name="${REPO_NAME}-${crate_dir}${PKG_SUFFIX:-}"
ffi_crate_link="[${ffi_crate_name} @ ${version}](${repo_url}/tree/${ffi_tag}/crates/ffi/${crate_dir})"

if [ -n "${BINDINGS_COMMIT:-}" ]; then
  echo "**Bindings commit:** [\`${BINDINGS_COMMIT:0:7}\`](${repo_url}/commit/${BINDINGS_COMMIT}) (on \`bindings\` branch)"
  echo
fi

echo "**FFI crate:** ${ffi_crate_link}"
echo

if [ ${#assets[@]} -eq 0 ]; then
  exit 0
fi

cat <<'HEADER'
## 📦 Assets by Platform

| Language | Platform | Architecture | Asset | Source |
| :--- | :--- | :--- | :--- | :--- |
HEADER

rust_link="[source](${repo_url}/tree/${tag}/crates/lib/${crate_dir})"
py_link="[source](${repo_url}/tree/${ffi_tag}/bindings/python/${py_pkg})"
go_link="[source](${repo_url}/tree/${go_tag}/bindings/go/${go_pkg})"
js_link="[source](${repo_url}/tree/${ffi_tag}/bindings/js/${js_pkg})"

rows_rust=()
rows_go=()
rows_js=()
rows_py_linux_aarch64=()
rows_py_linux_x86_64=()
rows_py_macos_arm64=()
rows_py_macos_x86_64=()
rows_py_windows=()
rows_other=()

for asset in "${assets[@]}"; do
  fname=$(basename "$asset")
  link="[${fname}](${repo_url}/releases/download/${tag}/${fname})"

  if echo "$fname" | grep -qi 'manylinux.*aarch64'; then
    rows_py_linux_aarch64+=("| **Python** | Linux | aarch64 | ${link} | ${py_link} |")
  elif echo "$fname" | grep -qi 'manylinux.*x86_64'; then
    rows_py_linux_x86_64+=("| **Python** | Linux | x86_64 | ${link} | ${py_link} |")
  elif echo "$fname" | grep -qi 'macosx.*arm64'; then
    rows_py_macos_arm64+=("| **Python** | macOS | Apple Silicon | ${link} | ${py_link} |")
  elif echo "$fname" | grep -qi 'macosx.*x86_64'; then
    rows_py_macos_x86_64+=("| **Python** | macOS | Intel | ${link} | ${py_link} |")
  elif echo "$fname" | grep -qi 'win.*amd64\|windows.*x86_64'; then
    rows_py_windows+=("| **Python** | Windows | x64 | ${link} | ${py_link} |")
  elif echo "$fname" | grep -qi '\.tgz$'; then
    rows_js+=("| **JS/TS** | All | All | ${link} | ${js_link} |")
  else
    rows_other+=("| — | — | — | ${link} | — |")
  fi
done

if $is_ffi; then
  rows_rust+=("| **Rust** | — | — | N/A | — |")
else
  cargo_add="\`cargo add --git ${repo_url} --tag ${tag} ${crate_dir}\`"
  rows_rust+=("| **Rust** | All | All | ${cargo_add} | ${rust_link} |")
fi

go_get="\`go get ${repo_url#https://}/bindings/go/${go_pkg}@${version}\`"
rows_go+=("| **Go** | All | All | ${go_get} | ${go_link} |")

for row in \
  "${rows_go[@]}" \
  "${rows_js[@]}" \
  "${rows_py_linux_aarch64[@]}" \
  "${rows_py_linux_x86_64[@]}" \
  "${rows_py_macos_arm64[@]}" \
  "${rows_py_macos_x86_64[@]}" \
  "${rows_py_windows[@]}" \
  "${rows_rust[@]}" \
  "${rows_other[@]}"; do
  echo "$row"
done
