# ADR: Separate FFI crates from library crates

**Status:** Accepted  
**Date:** 2026-11-06

## Context

Each Rust library crate under `crates/lib/<name>/` has a corresponding FFI crate under `crates/ffi/<name>/`. The FFI crates are thin wrappers that use UniFFI to expose the library's API through a C ABI, which `uniffi-bindgen` tools then use to generate language bindings (Python, Go, JavaScript).

The question arose whether this 1-to-1 separation introduces too much overhead, and whether the FFI code should live directly inside the library crates instead.

## Decision

Keep FFI crates separate from library crates.

## Rationale

### `crate-type` cannot be feature-gated

FFI crates require `crate-type = ["cdylib", "staticlib", "lib"]` in `Cargo.toml`. This field is static — Cargo does not support gating it behind a feature flag. Merging FFI into the library crate means every `cargo build` of the library also produces cdylib/staticlib outputs, and every pure-Rust consumer pulls in UniFFI proc macros as a transitive dependency. This is a meaningful compile-time and dependency cost for consumers who don't need FFI.

Conditional compilation (`#[cfg(feature = "ffi")]`) could gate the UniFFI scaffolding code itself, but the `crate-type` entries would still be present, and the conditional compilation complexity inside each crate would arguably be worse than maintaining separate crates.

### Pure Rust consumers get a clean dependency tree

With separation, a Rust project depending on `sev-core` gets only the library and its direct dependencies — no UniFFI, no proc macros, no FFI scaffolding. This matters for compile times and for keeping the dependency tree auditable.

### Independent versioning and release

FFI binding bugs (e.g., a missing export, a type mapping fix) can be fixed and released without bumping the library version. Conversely, internal library refactors that don't change the public API don't require an FFI release.

The overhead of keeping versions in sync is handled by an automated script (`scripts/sync-ffi-versions.sh`) that cascades library crate versions into their corresponding FFI crates on merge to `main`.

### The FFI crates are thin

Each FFI crate is a small wrapper — typically a `lib.rs` with UniFFI annotations re-exporting the library's public types. The per-crate maintenance cost (a `Cargo.toml` and a few source files) is low relative to the separation benefits.

## Alternatives considered

### Merge FFI into library crates with feature flags

Gate UniFFI dependencies and scaffolding behind a `ffi` feature. This eliminates the separate crates but introduces conditional compilation throughout the library code, and `crate-type` still cannot be feature-gated. Rejected because the complexity shifts rather than disappears, and pure-Rust consumers pay for it.

### Consolidate multiple FFI crates into fewer

Instead of one FFI crate per library crate, combine them into a single `sev-ffi` crate. This reduces per-crate overhead but sacrifices independent versioning and independent compilation — a change to one module's FFI layer forces a rebuild and re-release of all bindings. Worth reconsidering only if most releases consistently bump all crates together.

## Testing strategy

Correctness testing belongs in the library crates (`crates/lib/<name>/`). FFI crates should **not** duplicate those tests.

FFI crate tests exist only to verify the **type conversion boundary** — the `From` impls that translate between library types and FFI-compatible types. If an FFI crate is a pure passthrough (every exported function directly delegates to the library with no type conversion), it needs no tests; successful compilation already proves correctness.

An FFI crate should have tests when it:
- Defines FFI-specific types (`Complex`, `Matrix2x2`, `F64Pair`, `Point3`) with `From` impls for conversion to/from library types
- Maps error types between library and FFI representations (e.g., `core::MathError` → `MathError`)
- Constructs composite return types from library return values (e.g., wrapping a tuple into a `ComplexPair` record)

Tests should verify roundtrip fidelity (`FFI type → library type → FFI type` produces the original values) and that error variants map correctly. They should not re-test the underlying math or logic.

A CI check (`check-ffi-coverage` job) warns on PRs that modify `crates/lib/<name>/src/` without touching `crates/ffi/<name>/`, nudging developers to review whether the FFI layer needs updating.

## Consequences

- Developers must create a corresponding `crates/ffi/<name>/` when adding a new library crate that needs FFI bindings.
- Version syncing between library and FFI crates is automated via CI (`sync-versions` workflow) and does not require manual intervention.
- The binding generation pipeline can selectively build only the FFI crates that changed, reducing CI time.
