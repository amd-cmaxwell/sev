# sev
A monorepo refractoring of the [virtee/sev](https://github.com/virtee/sev) crate and providing modularity + Python, Go, and JS/WASM bindings for the different domains of the project. 

From the original Sev crate Readme:
> The `sev` crate provides an implementation of the [AMD Secure Encrypted
> Virtualization (SEV)][SEV] APIs and the [SEV Secure Nested Paging
> Firmware (SNP)][SNP] ABIs.

## Motivation

The `virtee/sev` crate provides Rust APIs for AMD SEV (Secure Encrypted Virtualization). To make these capabilities accessible from Python, Go, and JavaScript, we need a reliable FFI bridge. This project uses a simple math library as a testbed to work through the challenges of cross-language bindings before applying the approach to the real crate.

## What's here

A Rust workspace with six domain crates and a per-crate FFI layer that exposes functions via [Mozilla UniFFI](https://github.com/mozilla/uniffi-rs). Each FFI crate acts as a boundary layer — it defines UniFFI-compatible wrapper types and thin functions that delegate to the corresponding lib crate, keeping domain code free of FFI concerns. Bindings are generated for Python (via maturin), Go (via uniffi-bindgen-go), and JS/WASM (via uniffi-bindgen-react-native's `wasm2` mode).

### Key challenges explored

- **Types UniFFI can't cross directly** — tuples, fixed-size arrays, and tuple enum variants all required wrapper Record types (see [docs/ffi-conversion-pitfalls.md](docs/ffi-conversion-pitfalls.md))
- **Per-crate FFI separation** — each lib crate has its own FFI crate under `crates/ffi/`, enabling independent versioning and releases (see [docs/adr-ffi-crate-separation.md](docs/adr-ffi-crate-separation.md))
- **Python packaging with maturin** — maturin builds Rust, runs uniffi-bindgen, and produces a wheel in one step
- **Go bindings via uniffi-bindgen-go** — community tool from NordSecurity, static linking with CGo
- **JS/WASM bindings** — WASM compilation with TypeScript wrappers and npm packaging, verified in both Node.js and browser contexts
- **Cascading version pipeline** — lib crate version bumps automatically propagate to FFI crates with semver-aware bumping
- **Per-crate release workflow** — independent GitHub releases with per-language artifacts and rollback support

## Project structure

```
crates/
  lib/
    core/            Shared error types and basic arithmetic
    complex/         Complex number operations
    polynomials/     Polynomial evaluation, roots, derivatives
    linear-algebra/  Matrix operations, solvers, vector math
    trig/            Trigonometry, geometry, distance functions
    sequences/       Sequences, combinatorics, number theory
  ffi/
    core/            FFI wrapper for core
    complex/         FFI wrapper for complex
    polynomials/     FFI wrapper for polynomials
    linear-algebra/  FFI wrapper for linear-algebra
    trig/            FFI wrapper for trig
    sev/             FFI wrapper for the legacy Sev crate (empty for now)
    sequences/       FFI wrapper for sequences
    types/           Shared FFI types (no corresponding lib crate)
    tools/           Build tooling (excluded from workspace)

scripts/
  sync-ffi-versions.sh        Cascade lib version bumps to FFI crates
  check-ffi-coverage.sh       Verify FFI source changes for lib API changes
  check-ffi-versions.sh       Validate FFI versions >= lib versions
  sync-workspace-versions.sh  Sync root Cargo.toml workspace dep versions
  check-workspace-versions.sh Validate workspace dep versions match crates
  release-body.sh             Generate GitHub release notes with asset tables

examples/
  py-demo/           Python smoke test
  go-demo/           Go smoke test
  js-demo/           JS smoke test (Node.js)
  web-demo/          Browser smoke test (Vite + vanilla TS)

docs/                Design decisions and pitfall documentation
```

## Setup

```bash
make setup    # configures git hooks and installs build tools
```

This sets up the `.githooks/` directory as the git hooks path. The pre-commit hook automatically:
1. Syncs FFI crate versions when lib versions change (semver-aware cascading)
2. Checks that lib crate source changes have corresponding FFI updates
3. Syncs workspace dependency versions in the root `Cargo.toml`
4. Validates FFI and workspace version alignment

### Repository variables

When creating a new project from this template, configure these as GitHub repository variables (Settings > Secrets and variables > Actions > Variables):

| Variable | Description |
|---|---|
| `REPO_OWNER` | GitHub owner/org (used for package naming) |
| `REPO_NAME` | Repository name |
| `PKG_SUFFIX` | Package name suffix (e.g. `-ffi`) |
| `NPM_SCOPE` | npm scope for JS packages (e.g. `@my-org`) |

The Makefile resolves these dynamically from the git remote by default. Override `REPO_SLUG`, `REPO_OWNER`, `REPO_NAME`, `PKG_SUFFIX`, or `NPM_SCOPE` as needed.

## Building bindings

```bash
make generate-python    # Build Python wheels for all FFI crates
make generate-go        # Generate Go modules with CGO static libs
make generate-js        # Build WASM + TS bindings and pack npm tarballs
make generate-all       # All of the above
```

## Testing

```bash
make test               # Run Rust tests
make test-python        # Run Python smoke test (examples/py-demo)
make test-go            # Run Go smoke test (examples/go-demo)
make test-js            # Run JS smoke test (examples/js-demo)
make test-web           # Build browser demo with Vite (examples/web-demo)
make test-all           # All binding tests
make clean-web          # Remove web-demo generated files
```

The `web-demo` is a browser-based interactive calculator that exercises every math module via WASM. It also runs the same smoke tests as `js-demo` on page load. After `make test-web`, you can open it locally:

```bash
cd examples/web-demo && npx vite
```

## CI / CD

### CI (`ci.yml`)

Runs on push to main, pull requests, and manual dispatch. Validates FFI coverage and version alignment, runs Rust tests on Linux/macOS/Windows, builds all three binding flavors, and runs smoke tests for each.

### Release (`release.yml`)

Triggered manually via `workflow_dispatch`. Supports optional inputs to scope the release to specific crates:
- **`crates`** — space-separated lib crate names (e.g. `core trig`)
- **`ffi-libs`** — space-separated FFI lib names

When inputs are empty, all changed crates are released (default).

The release flow:
1. Verifies CI passed on HEAD
2. Detects version changes by comparing against git tags
3. Builds Python, Go, and JS bindings for changed FFI crates
4. Commits bindings to the `bindings` branch and tags them
5. Creates per-crate GitHub releases with artifact tables
6. Rolls back tags, releases, and the bindings branch on failure

#### Versioning

- **Lib crates** are tagged as `crates/{name}/v{version}`
- **FFI crates** are tagged as `crates/ffi/{name}/v{version}` (Rust release) and `ffi/{name}/v{version}` (bindings)
- **Go modules** are tagged as `bindings/go/{pkg}/v{version}`

When a lib crate version changes, `sync-ffi-versions.sh` automatically bumps the corresponding FFI crate:
- If the FFI version is behind the lib version, it matches the lib version
- If the FFI version is already at or ahead, it gets a patch bump
- The sync is idempotent across pre-commit hook re-runs

FFI-only releases (no lib change) are distinguished with a `-ffi` title suffix and a release body notice.

## Dependencies

- Rust toolchain (for building FFI crates)
- [maturin](https://www.maturin.rs/) (for Python packaging)
- [uniffi-bindgen-go](https://github.com/NordSecurity/uniffi-bindgen-go) (for Go bindings)
- Node.js >= 20, pnpm (for JS/WASM bindings)
- Python >= 3.8, Go >= 1.21

## Next steps

Apply this approach to [virtee/sev](https://github.com/virtee/sev) to provide Python, Go, and JS interfaces for AMD SEV attestation and key management.