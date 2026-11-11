# Bundle WASM bindings as per-crate npm packages

## Context

Each FFI crate needs its own npm package under `<npm-scope>/`, mirroring the monorepo structure (same as Python gets per-crate wheels and Go gets per-crate packages). The generated TS uses relative `./sev_types_ffi` imports for cross-module deps — these need rewriting to npm package names during packaging.

Two existing bugs to fix: (1) `linear_algebra` and `trig` need `--crate-name` passed to the bindgen, and (2) cross-module imports need the types `.ts`/`.wasm` alongside or rewritten.

## Package structure (per crate)

```
bindings/wasm/<npm-scope>/sev-core-ffi/
  package.json
  tsconfig.json
  src/
    sev_core_ffi.ts    (generated)
    sev_core_ffi.wasm  (generated)
    uniffi_runtime.ts             (generated)
  dist/                           (compiled .js + .d.ts)
```

For crates that import types (complex, polynomials, sequences, linear_algebra, trig):
```
bindings/wasm/<npm-scope>/sev-complex-ffi/
  package.json                    (has dependency on <npm-scope>/sev-types-ffi)
  tsconfig.json
  src/
    sev_complex_ffi.ts (import rewritten to '<npm-scope>/sev-types-ffi')
    sev_complex_ffi.wasm
    uniffi_runtime.ts
  dist/
```

## Changes

### 1. Fix `generate-wasm` in Makefile

- Add `--crate-name $$pkg` to each invocation (fixes linear_algebra/trig)
- Keep per-module output dirs but flatten each to just `src/` inside the package dir

### 2. Add `package-wasm` Makefile target

For each FFI lib:
1. Generate into a temp dir, then move into `$(WASM_BINDINGS_DIR)/<npm-scope>/sev-<crate>-ffi/src/`
2. Stamp `package.json` from template (substitute `PKG_NAME` and add deps for types if needed)
3. Copy `tsconfig.json` template
4. Rewrite any `from './sev_types_ffi'` imports to `from '<npm-scope>/sev-types-ffi'`
5. Run `npx tsc` in each package dir to produce `dist/`

### 3. New templates

**`templates/js/package.json`:**
```json
{
  "name": "PKG_NAME",
  "version": "0.0.0",
  "type": "module",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js"
    }
  },
  "files": ["dist/", "src/*.wasm"]
}
```

**`templates/js/tsconfig.json`:** (same for all, no templating needed)
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ES2022",
    "moduleResolution": "bundler",
    "declaration": true,
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "skipLibCheck": true
  },
  "include": ["src/**/*.ts"]
}
```

### 4. Index file generation

For each package, generate `src/index.ts`:
```typescript
export * from './sev_core_ffi.js';
```

### 5. Update `generate-wasm` to use `--crate-name`

### 6. Update CI (`ci.yml`)

Update `js-bindings` job to run `make package-wasm` and verify compiled output.

### 7. Update Release (`release.yml`)

- `build-js-bindings`: run `make package-wasm`, `npm pack` each package
- Upload tarballs as artifacts
- `release` job: attach npm tarballs to GitHub release

## Files to modify
- `Makefile` — fix `generate-wasm` (add `--crate-name`), add `package-wasm`
- `templates/js/package.json` — new
- `templates/js/tsconfig.json` — new
- `.github/workflows/ci.yml` — update js-bindings job
- `.github/workflows/release.yml` — update build/release jobs

## Verification
1. `make generate-wasm` — each module dir has correct `.ts` with own-name exports
2. `make package-wasm` — each `dist/` has `.js` + `.d.ts`, cross-module imports rewritten
3. `cd bindings/wasm/<npm-scope>/sev-core-ffi && npm pack` produces valid tarball
