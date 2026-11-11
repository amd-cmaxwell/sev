# UniFFI Bindings Summary

## FFI Crate (`crates/ffi/`)

Completely rewritten with UniFFI proc-macros. No internal crates were modified — all bridging happens here.

### Types (`crates/ffi/src/types.rs`)

9 FFI types defined with UniFFI derives:

| Type | Kind | Purpose |
|------|------|---------|
| `MathError` | Error enum | Maps `core::MathError` with struct variant for `InvalidInput { message }` |
| `Complex` | Record | Complex number with `re`, `im` fields |
| `ComplexPair` | Record | Replaces `(Complex, Complex)` tuple returns |
| `F64Pair` | Record | Replaces `(f64, f64)` tuple returns |
| `Point2` | Record | 2D point/vector, replaces `[f64; 2]` |
| `Point3` | Record | 3D point/vector, replaces `[f64; 3]` |
| `Matrix2x2` | Record | 2x2 matrix with flat fields `m00..m11`, replaces `[[f64; 2]; 2]` |
| `Matrix3x3` | Record | 3x3 matrix with flat fields `m00..m22`, replaces `[[f64; 3]; 3]` |

### Wrapper Modules

52 exported functions across 6 modules:

| Module | Functions | Source crate |
|--------|-----------|-------------|
| `arithmetic.rs` | `add` | `sev-core` |
| `complex.rs` | `complex_new`, `complex_from_polar`, `complex_magnitude`, `complex_argument`, `complex_conjugate`, `complex_add`, `complex_sub`, `complex_mul`, `complex_div`, `complex_display`, `quadratic_roots_complex` | `sev-complex` |
| `polynomials.rs` | `polynomial_evaluate`, `quadratic_roots`, `linear_root`, `polynomial_add`, `polynomial_multiply`, `polynomial_derivative` | `sev-polynomials` |
| `linear_algebra.rs` | `determinant_2x2`, `determinant_3x3`, `solve_2x2`, `solve_3x3`, `matrix_multiply_2x2`, `matrix_inverse_2x2`, `transpose_2x2`, `dot_product`, `cross_product_3d` | `sev-linear-algebra` |
| `trig.rs` | `degrees_to_radians`, `radians_to_degrees`, `law_of_cosines_side`, `law_of_cosines_angle`, `law_of_sines_side`, `distance_2d`, `distance_3d`, `midpoint_2d`, `triangle_area_heron`, `circle_area`, `circle_circumference` | `sev-trig` |
| `sequences.rs` | `arithmetic_term`, `arithmetic_sum`, `geometric_term`, `geometric_sum`, `geometric_sum_infinite`, `factorial`, `permutations`, `combinations`, `gcd`, `lcm`, `fibonacci`, `is_prime` | `sev-sequences` |

## Python Bindings (`bindings/python/`)

- Generated `.py` module + `.so` shared library
- Errors raised as `MathError.DivisionByZero`, `MathError.InvalidInput`, etc.
- Demo: `examples/demo.py`

## Go Bindings (`bindings/go/`)

- Generated `.go` + `.h` + `.a` static library
- Functions exported as PascalCase (e.g., `ComplexMagnitude`, `QuadraticRoots`)
- Errors returned as idiomatic `(T, error)` pairs
- Demo: `bindings/go/demo.go`

## Regeneration Commands

```bash
# Build the FFI crate
cargo build --release -p sev-ffi

# Generate Python bindings
cargo run --release --bin uniffi-bindgen -- generate \
    --library target/release/libsev_ffi.so \
    --language python \
    --out-dir bindings/python
cp target/release/libsev_ffi.so bindings/python/

# Generate Go bindings
uniffi-bindgen-go \
    --library target/release/libsev_ffi.so \
    --out-dir bindings/go
cp target/release/libsev_ffi.a bindings/go/sev_ffi/
```

Note: After regenerating Go bindings, add the CGo LDFLAGS directive to the generated `.go` file:
```
// #cgo LDFLAGS: -L${SRCDIR} -lsev_ffi -lm -ldl -lpthread
```

## Version Dependencies

- `uniffi` Rust crate: v0.29
- `uniffi-bindgen-go`: v0.5.0+v0.29.5 (from [NordSecurity/uniffi-bindgen-go](https://github.com/NordSecurity/uniffi-bindgen-go))
