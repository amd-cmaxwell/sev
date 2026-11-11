# Plan: GitHub Actions CI/CD for Python and Go bindings

## Context

No CI exists. The build pipeline is entirely manual via the Makefile. We want GitHub Actions that build and test both bindings across Linux/macOS/Windows, and publish artifacts on tagged releases to GitHub Releases.

## Files to create

1. **`.github/workflows/ci.yml`** — build + test on push to main and PRs
2. **`.github/workflows/release.yml`** — build platform wheels + attach to GitHub Release on `v*` tags

---

## `.github/workflows/ci.yml`

**Triggers**: `push: branches: [main]` + `pull_request: branches: [main]`

### Job 1: `rust-test` (matrix: ubuntu/macos/windows)
- `dtolnay/rust-toolchain@stable`
- `Swatinem/rust-cache`
- `cargo test --workspace`

### Job 2: `python-bindings` (matrix: ubuntu/macos/windows, needs: rust-test)
- Install Rust (stable), Python 3.12, maturin (`pip install maturin`)
- Set `LIB_EXT` env var per platform (so/dylib/dll)
- `cargo build --release -p sev-ffi`
- Run uniffi-bindgen to generate raw Python: `cargo run --release --bin uniffi-bindgen -- generate --library target/release/libsev_ffi.$LIB_EXT --language python --out-dir $RUNNER_TEMP/uniffi-python-gen`
- Run module generator: `python3 bindings/python/scripts/generate_modules.py $RUNNER_TEMP/uniffi-python-gen/sev_ffi.py`
- Build wheel: `cd bindings/python && maturin build --release --out $RUNNER_TEMP/wheels`
- Install and smoke test: `pip install $RUNNER_TEMP/wheels/*.whl && python examples/demo.py`
- Upload wheel artifact via `actions/upload-artifact`

### Job 3: `go-bindings` (matrix: ubuntu/macos, needs: rust-test)
- **Skip Windows**: uniffi-bindgen-go produces CGo code needing gcc + static linking, which is fragile on Windows. The Go bindings are consumed as a module (users `go get` from the repo), not as a platform binary.
- Install Rust (stable), Go 1.21, Python 3.x
- Install uniffi-bindgen-go: `cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.5.0+v0.29.5`
- `cargo build --release -p sev-ffi`
- Generate Go bindings: `uniffi-bindgen-go --library target/release/libsev_ffi.$LIB_EXT --out-dir $RUNNER_TEMP/uniffi-go-gen`
- Copy files to `bindings/go/sev_ffi/`
- Patch `#cgo LDFLAGS` using a portable Python one-liner (not `sed -i` which differs macOS vs Linux):
  ```python
  import pathlib, sys
  p = pathlib.Path(sys.argv[1])
  t = p.read_text()
  t = t.replace(
      '// #include <sev_ffi.h>',
      '// #cgo LDFLAGS: -L${SRCDIR} -lsev_ffi -lm -ldl -lpthread\n// #include <sev_ffi.h>'
  )
  p.write_text(t)
  ```
  (On macOS: drop `-ldl` since it doesn't exist — or keep it and let the linker silently ignore via `-Wl,--as-needed`... actually macOS linker errors on `-ldl`. Handle via platform conditional.)
- Run module generator: `python3 bindings/go/scripts/generate_modules.py bindings/go/sev_ffi/sev_ffi.go`
- `cd bindings/go && go build ./...`
- Smoke test: `cd bindings/go && go run ./example/demo/`

### Platform-specific linker flags
| Platform | LDFLAGS |
|----------|---------|
| Linux    | `-lm -ldl -lpthread` |
| macOS    | `-lm -lpthread -framework Security -framework Foundation` |

The CGo LDFLAGS patch step will use a platform-conditional value.

### Caching
- Rust: `Swatinem/rust-cache` on all jobs
- Go: `actions/setup-go` with `cache: true`
- Python: `actions/setup-python` with `cache: pip`

---

## `.github/workflows/release.yml`

**Triggers**: `push: tags: ['v*']`

### Job 1: `build-wheels` (matrix: ubuntu/macos/windows)
- Same steps as `python-bindings` in CI
- Upload platform wheel as artifact

### Job 2: `release` (needs: build-wheels, runs-on: ubuntu-latest)
- Download all wheel artifacts
- Create GitHub Release with `softprops/action-gh-release`
- Attach all `.whl` files

---

## Key decisions

- **Go bindings skip Windows** in CI: CGo cross-compilation on Windows GHA runners is unreliable, and Go consumers `go get` the module (they compile locally with their own toolchain). Linux + macOS coverage is sufficient.
- **Portable patching**: Replace `sed -i` with a Python script for the CGo LDFLAGS injection — works identically on all platforms.
- **uniffi-bindgen-go installed via cargo**: `cargo install --git ... --tag v0.5.0+v0.29.5`. Cached by rust-cache.

## Verification

- Push a branch with the workflow files and open a PR — CI runs automatically
- Check all matrix cells pass (3 for Rust, 3 for Python, 2 for Go)
- Create a test tag `v0.0.0-test` to verify the release workflow
