# UniFFI Python & Go Bindings for Math Monorepo

## Context

The workspace has 6 domain crates (core, complex, polynomials, linear-algebra, trig, sequences) and a mostly-empty FFI crate. The goal is to expose all ~38 public functions via Mozilla UniFFI, generating both Python and Go bindings. No internal crates will be modified — all FFI bridging happens in `crates/ffi`.

## Key design decisions

- **Proc-macro approach** (not UDL) — annotate with `#[uniffi::export]`, `#[derive(uniffi::Record)]`, etc.
- **Wrapper pattern** — FFI crate defines its own UniFFI-compatible types and thin wrapper functions that delegate to internal crates. This avoids coupling domain crates to UniFFI.
- **Complex as Record + free functions** — simpler than Object, avoids Arc overhead and Go's Destroy() requirement. Fields directly accessible.
- **UniFFI v0.31** with `uniffi-bindgen-go` v0.7.1+v0.31.0 for Go bindings.

## UniFFI type constraints driving wrapper types

| Unsupported in UniFFI | Where it appears | FFI replacement |
|---|---|---|
| Tuples `(f64, f64)` | polynomials::quadratic_roots, trig::midpoint_2d | `F64Pair` / `Point2` Records |
| Tuples `(Complex, Complex)` | complex::quadratic_roots_complex | `ComplexPair` Record |
| Fixed arrays `[[f64;2];2]` | all linear-algebra matrix fns | `Matrix2x2` Record |
| Fixed arrays `[[f64;3];3]` | determinant_3x3, solve_3x3 | `Matrix3x3` Record |
| Fixed arrays `[f64;2]`, `[f64;3]` | solve_2x2/3x3 results, cross_product_3d | `Point2` / `Point3` Records |
| Tuple enum variant `InvalidInput(String)` | core::MathError | Struct variant `InvalidInput { message }` |

## Implementation steps

### 1. Update `crates/ffi/Cargo.toml`

- Add `[lib]` section: `crate-type = ["cdylib", "staticlib", "lib"]`
- Add `uniffi = { version = "0.31", features = ["cli"] }` dependency
- Add all 6 domain crates as dependencies
- Add `[[bin]]` for uniffi-bindgen at `src/bin/uniffi-bindgen.rs`

### 2. Create uniffi-bindgen binary

New file `crates/ffi/src/bin/uniffi-bindgen.rs` — 3-line main calling `uniffi::uniffi_bindgen_main()`.

### 3. Create FFI module structure

```
crates/ffi/src/
  lib.rs              — uniffi::setup_scaffolding!() + mod declarations
  types.rs            — 8 Records + 1 Error enum (MathError, Complex, ComplexPair, F64Pair, Point2, Point3, Matrix2x2, Matrix3x3)
  conversions.rs      — From impls between FFI types ↔ internal types
  arithmetic.rs       — 1 fn: add (renamed from core.rs to avoid Rust prelude conflict)
  complex.rs          — 11 fns: complex_new, complex_from_polar, complex_magnitude, complex_argument, complex_conjugate, complex_add/sub/mul/div, complex_display, quadratic_roots_complex
  polynomials.rs      — 6 fns: polynomial_evaluate, quadratic_roots, linear_root, polynomial_add/multiply/derivative
  linear_algebra.rs   — 9 fns: determinant_2x2/3x3, solve_2x2/3x3, matrix_multiply/inverse/transpose_2x2, dot_product, cross_product_3d
  trig.rs             — 11 fns: degrees_to_radians, radians_to_degrees, law_of_cosines_side/angle, law_of_sines_side, distance_2d/3d, midpoint_2d, triangle_area_heron, circle_area/circumference
  sequences.rs        — 14 fns: arithmetic_term/sum, geometric_term/sum/sum_infinite, factorial, permutations, combinations, gcd, lcm, fibonacci, is_prime
```

**Total: 52 exported FFI functions, 9 types.**

### 4. Wrapper function pattern

Each wrapper is a `#[uniffi::export]` function that:
1. Takes UniFFI-compatible types (Records, Vec, primitives)
2. Converts to internal types via From impls
3. Calls the internal function
4. Converts result back, mapping errors with `.map_err(MathError::from)`

Example:
```rust
#[uniffi::export]
pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Result<F64Pair, MathError> {
    let (r1, r2) = sev_polynomials::quadratic_roots(a, b, c).map_err(MathError::from)?;
    Ok(F64Pair { first: r1, second: r2 })
}
```

Slice params (`&[f64]`) become `Vec<f64>` in FFI signatures; wrappers pass `&v` to internal fns.

### 5. Generate Python bindings

```bash
cargo build --release -p sev-ffi
cargo run --release --bin uniffi-bindgen -- generate \
    --library target/release/libsev_ffi.so \
    --language python --out-dir bindings/python
cp target/release/libsev_ffi.so bindings/python/
```

### 6. Generate Go bindings

```bash
cargo install uniffi-bindgen-go \
    --git https://github.com/NordSecurity/uniffi-bindgen-go \
    --tag v0.7.1+v0.31.0
uniffi-bindgen-go \
    --library target/release/libsev_ffi.so \
    --out-dir bindings/go
cp target/release/libsev_ffi.a bindings/go/sev_ffi/
```

### 7. Demo scripts

- `examples/demo.py` — exercises all function groups, demonstrates error handling
- `examples/demo.go` — same coverage in Go

## Verification

1. `cargo test --workspace` — existing tests still pass (no internal changes)
2. `cargo build --release -p sev-ffi` — FFI crate compiles
3. Generate Python bindings and run `python3 examples/demo.py`
4. Generate Go bindings and run `go run examples/demo.go`
