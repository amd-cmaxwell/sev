# Add WASM/TypeScript Bindings via uniffi-bindgen-js

## Context

The project has Python and Go bindings generated via uniffi. We're adding WASM/TypeScript bindings using [uniffi-bindgen-js](https://github.com/nchapman/uniffi-bindgen-js). This requires bumping uniffi from 0.29.5 to 0.31.0 (uniffi-bindgen-js v0.2.0 requires it; uniffi-bindgen-go v0.7.1 also supports 0.31.0, so everything aligns).

## Pass 1: Bump uniffi to 0.31.0

Update uniffi version across all FFI crates and the Go bindgen dependency.

**Files to modify:**
- `crates/ffi/types/Cargo.toml` — `uniffi = "0.31"`, update `uniffi-bindgen-go` tag to `v0.7.1+v0.31.0`
- `crates/ffi/{core,complex,polynomials,linear-algebra,trig,sequences}/Cargo.toml` — `uniffi = "0.31"` (6 files, same change)

**Verify:** `cargo build --release` and `cargo test --workspace` pass.

## Pass 2: Add uniffi-bindgen-js binary

Follow the same pattern as the existing Go bindgen: add as a git dependency behind the `ffi` feature, with a thin binary wrapper.

**`crates/ffi/types/Cargo.toml`:**
- Add `uniffi-bindgen-js` git dependency (tag `v0.2.0`, optional)
- Update `ffi` feature to include `dep:uniffi-bindgen-js`
- Add `[[bin]]` entry for `uniffi-bindgen-js` with `required-features = ["ffi"]`

**New file: `crates/ffi/types/src/bin/uniffi-bindgen-js.rs`:**
```rust
fn main() -> anyhow::Result<()> {
    let cli = uniffi_bindgen_js::cli::Cli::parse();
    uniffi_bindgen_js::run(cli)
}
```
Note: needs `clap` and `anyhow` as dependencies of the types-ffi crate (or just use the re-exports). Check if uniffi-bindgen-js re-exports what we need.

**`uniffi.toml`** — add `[bindings.js]` section (external packages config, similar to python):
```toml
[bindings.js]
external_packages = { sev_types_ffi = "./sev_types_ffi" }
```

**Verify:** `cargo build --release -p sev-types-ffi --features ffi --bin uniffi-bindgen-js`

## Pass 3: Add `generate-wasm` Makefile target + bindings/wasm directory

**`Makefile`:**
- Add `WASM_BINDINGS_DIR ?= $(BINDINGS_DIR)/wasm`
- Add `generate-wasm` target that:
  1. Builds all FFI crates for `wasm32-unknown-unknown` target: `cargo build --release --target wasm32-unknown-unknown` (needs the wasm32 target installed)
  2. For each FFI lib, runs `uniffi-bindgen-js generate` with the `.wasm` file as input:
     ```
     cargo run --release -p sev-types-ffi --features ffi --bin uniffi-bindgen-js -- \
       generate target/wasm32-unknown-unknown/release/sev_${mod}_ffi.wasm \
       --out-dir $(WASM_BINDINGS_DIR)/sev_${mod}_ffi
     ```
  3. Note: uniffi-bindgen-js reads from `.wasm` files (not `.so`/`.dylib`)
- Add `generate-wasm` to `generate-all`

**New directory:** `bindings/wasm/` — created by the Makefile target. Add a `.gitkeep` or let CI create it.

**Verify:** `make generate-wasm` produces TypeScript files in `bindings/wasm/`.

## Pass 4: Update CI workflow (ci.yml)

Add a `js-bindings` job following the pattern of `python-bindings` and `go-bindings`:

```yaml
js-bindings:
  name: WASM bindings
  needs: rust-test
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-unknown-unknown
    - uses: Swatinem/rust-cache@v2
    - name: Build WASM FFI crates
      run: cargo build --release --target wasm32-unknown-unknown
    - name: Generate WASM bindings
      run: |
        for mod in $FFI_MODULES; do
          echo "  generating wasm bindings for $mod"
          cargo run --release -p sev-types-ffi --features ffi --bin uniffi-bindgen-js -- \
            generate "target/wasm32-unknown-unknown/release/sev_${mod}_ffi.wasm" \
            --out-dir "$RUNNER_TEMP/uniffi-wasm-gen/sev_${mod}_ffi"
        done
    - name: Verify generated files
      run: find "$RUNNER_TEMP/uniffi-wasm-gen" -type f | head -50
```

Single platform (ubuntu) since WASM is platform-independent.

## Pass 5: Update Release workflow (release.yml)

**Add `build-js-bindings` job:**
- Single platform (ubuntu-latest)
- Install `wasm32-unknown-unknown` target
- Run `make generate-wasm`
- Upload `bindings/wasm/` as artifact

**Update `publish-bindings`:**
- Add download step for wasm artifacts
- Add `bindings/wasm/` to the git add/commit

**Update `release` job:**
- Add wasm bindings as release assets (tar.gz the `bindings/wasm/` directory)
- Update `needs` arrays to include `build-js-bindings`

## Special considerations

- **wasm32-unknown-unknown target:** CI needs `rustup target add wasm32-unknown-unknown`. The `dtolnay/rust-toolchain` action supports a `targets` input.
- **uniffi features for WASM:** The FFI crates may need additional uniffi features like `wasm-unstable-single-threaded` and `scaffolding-ffi-buffer-fns` for WASM compilation. These would go on the uniffi dependency in each FFI crate. We'll test if the default features work first.
- **No Windows WASM:** WASM is cross-compiled, so only need one CI runner.
- **clap/anyhow deps:** The bindgen-js binary needs `clap::Parser` and `anyhow::Result`. We'll add these to the types-ffi crate deps behind the `ffi` feature.

## Verification

1. `cargo test --workspace` — all existing tests pass after uniffi bump
2. `cargo build --release --target wasm32-unknown-unknown` — WASM compilation works
3. `make generate-wasm` — TypeScript bindings are generated in `bindings/wasm/`
4. Check generated `.ts` files look reasonable
