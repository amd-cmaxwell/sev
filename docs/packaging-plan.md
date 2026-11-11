# Package Python & Go Bindings for Distribution

## Context

The UniFFI bindings are generated but live as raw output files — no packaging metadata, no proper module structure. Both Python and Go packages should be named `sev_ffi` and be installable via their language's standard tooling.

## Python Package (via maturin)

Maturin has native UniFFI support — it compiles the Rust crate, runs uniffi-bindgen internally,
bundles the .so, and produces a platform-tagged wheel in a single step. No manual file management needed.

### Target structure

```
bindings/python/
  pyproject.toml                    (maturin build backend, points to crates/ffi/Cargo.toml)
```

That's it. Maturin generates the Python module and bundles the native library at build time.
No `setup.py`, `MANIFEST.in`, `__init__.py`, or pre-generated `.py` files are checked in.

### pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "sev-ffi"
version = "0.1.0"
description = "Python bindings for the sev math library"
requires-python = ">=3.8"

[tool.maturin]
bindings = "uniffi"
manifest-path = "../../crates/ffi/Cargo.toml"
module-name = "sev_ffi"
```

### Commands

```bash
# Install locally for development (editable)
cd bindings/python && maturin develop --release

# Build a distributable wheel
cd bindings/python && maturin build --release
# Output: target/wheels/sev_ffi-0.1.0-py3-none-linux_x86_64.whl

# Run the demo
python examples/demo.py
```

### How it works

1. Maturin reads `pyproject.toml`, finds `bindings = "uniffi"` and `manifest-path`
2. Compiles the Rust `crates/ffi` cdylib in release mode
3. Runs the `uniffi-bindgen` binary (from the crate's `[[bin]]` target) to generate Python code
4. Bundles the generated `.py` and compiled `.so` into a platform-tagged wheel
5. `maturin develop` installs the wheel into the active virtualenv

## Go Package

### Target structure

```
bindings/go/
  go.mod                             (module github.com/<owner>/<repo>/bindings/go)
  Makefile
  README.md
  example/
    demo/
      main.go                        (moved from demo.go, updated import path)
  sev_ffi/
    sev_ffi.go            (unchanged)
    sev_ffi.h             (unchanged)
    libsev_ffi.a          (unchanged)
```

### Steps

1. **Update `go.mod`** — change module path from `sev-go` to `github.com/<owner>/<repo>/bindings/go`
2. **Move `demo.go`** to `example/demo/main.go` — keeps module root clean as a library, not an executable
3. **Update import path** in the moved demo from `sev-go/sev_ffi` to `github.com/<owner>/<repo>/bindings/go/sev_ffi`
4. **Create `Makefile`** — targets to rebuild .a from Rust source and run demo
5. **Create `README.md`** — usage, import path, prerequisites

### Go module versioning note

For `go get` to work, tags must follow subdirectory convention: `bindings/go/v0.1.0`. Without a tag, users reference commit hashes.

### Verification

```bash
cd bindings/go && go build ./sev_ffi/
cd bindings/go && go run ./example/demo/
```

## Root-level cleanup

- **Add `.gitignore` entries** for `__pycache__/`, `*.egg-info/`, `build/`, `dist/`
- **Create root `Makefile`** with targets: `build`, `generate-python`, `generate-go`, `test-python`, `test-go`
- The `generate-python` target runs `maturin develop --release` in `bindings/python/`
- The `generate-go` target runs `uniffi-bindgen-go`, copies files, and auto-injects the CGo LDFLAGS directive via `sed`
