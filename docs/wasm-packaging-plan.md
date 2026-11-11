# WASM/TypeScript Bindings — pnpm Workspace Packaging

## Context

Each FFI crate gets its own npm package under `<npm-scope>/`, mirroring the monorepo structure. Packages are installable via pnpm's git subdirectory support:

```
pnpm install git+https://github.com/<owner>/<repo>#path=bindings/wasm/<npm-scope>/sev-core-ffi
```

CI commits only source files — no `dist/`, no `node_modules/`. A `prepare` script compiles TypeScript on install.

## Tooling

- **uniffi-bindgen-react-native** (v0.31.0-6) — `wasm2` flavor generates TypeScript bindings from compiled `.wasm` files
- **Runtime deps:** `@ubjs/core` and `@ubjs/wasm` (v0.31.0-6) — required by generated code
- **uniffi-runtime-wasm** (v0.31.0-6) — Rust crate required at compile time for WASM targets

### uniffi-bindgen-js bugs (abandoned)

The project initially used uniffi-bindgen-js (v0.2.0), which was abandoned due to these bugs:

1. `--crate-name` required for multi-crate `.wasm` files — without it, only the first/types crate's bindings are generated
2. Missing `_liftErrorMathError` — generator emits calls but never defines the function for external crate error types
3. Missing `_liftPoint2`/`_lowerPoint2` — generator emits calls but never defines lifting/lowering functions for external crate record types

These are upstream bugs in https://github.com/nicholasChapman/uniffi-bindgen-js.

## Package structure (committed to repo)

```
bindings/wasm/
  package.json              (private workspace root)
  pnpm-workspace.yaml       (lists <npm-scope>/*)
  <npm-scope>/sev-core-ffi/
    package.json            (has "prepare": "tsc")
    tsconfig.json
    src/
      index.ts              (generated — re-exports + init)
      sev_core_ffi.ts
      sev_core_ffi-ffi.ts
      sev_core_ffi.wasm
```

Crates with types dependency (complex, polynomials, sequences, linear_algebra, trig) include the types `.ts` files inline — the generator bundles them, so no cross-package npm dependencies are needed.

**NOT committed:** `dist/`, `node_modules/`, `pnpm-lock.yaml`

## Build flow

1. `make generate-wasm` — builds each crate for `wasm32-unknown-unknown`, runs `uniffi-bindgen-react-native generate wasm2 bindings`, copies `.wasm` into `src/`
2. `make package-wasm` — stamps `package.json`/`tsconfig.json` from templates, installs deps, compiles with `tsc`
3. Release: `npm pack` each package → `.tgz` tarballs attached to GitHub release

## Rust requirements

Each FFI crate needs:
- `wasm` feature enabling `uniffi/wasm-unstable-single-threaded`, `uniffi/scaffolding-ffi-buffer-fns`, `dep:uniffi-runtime-wasm`
- `#[cfg(target_arch = "wasm32")] extern crate uniffi_runtime_wasm as _;` in `lib.rs`
- Crates depending on `sev-types-ffi` must propagate the `wasm` feature
