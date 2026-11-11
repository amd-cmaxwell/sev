# Move core crates into `crates/lib/`

## Context

CI and githooks should autodiscover new crates that may need matching FFI crates.
Currently, non-FFI crates live at `crates/{name}` alongside `crates/ffi/`, making
glob-based discovery ambiguous. Moving them to `crates/lib/` lets the root
`Cargo.toml` use `crates/lib/*` as a workspace glob, and scripts can discover core
crates by scanning `crates/lib/`.

## Crates being moved

`complex`, `core`, `linear-algebra`, `polynomials`, `sequences`, `trig`

From `crates/{name}` → `crates/lib/{name}`

## Files to update

### Root `Cargo.toml`
- Replace 6 explicit workspace members with glob `"crates/lib/*"`
- Update 6 workspace dependency paths: `crates/{name}` → `crates/lib/{name}`

### Scripts
- `scripts/check-workspace-versions.sh` — `crates/*/` → `crates/lib/*/`
- `scripts/sync-workspace-versions.sh` — same pattern
- `scripts/check-ffi-versions.sh` — lib_toml path construction
- `scripts/check-ffi-coverage.sh` — globs and path construction
- `scripts/sync-ffi-versions.sh` — loop, skip logic, tag paths
- `scripts/release-body.sh` — lib crate path construction

### CI workflows
- `.github/workflows/release.yml` — lib crate glob and path construction
- `.github/workflows/check-workspace-versions.yml` — comment only

### Documentation
- `README.md` — project structure diagram
- `docs/adr-ffi-crate-separation.md` — lib crate path reference
- `docs/per-crate-release-plan.md` — tag pattern reference
- `crates/README.md` — directory structure

### No changes needed
- `.githooks/pre-commit` (only refs `crates/ffi/`)
- `Makefile` (only refs `crates/ffi/`)
- Individual crate `Cargo.toml` files (all use `workspace = true` deps)
- FFI crate `Cargo.toml` files (all use workspace deps)
- Build CI workflows (delegate to scripts/Makefile)

## Git tag impact

Existing tags use `crates/{name}/v{version}`. New tags will use
`crates/lib/{name}/v{version}`. Old tags remain valid as historical markers.
