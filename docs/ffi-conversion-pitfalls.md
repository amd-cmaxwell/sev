# FFI Conversion Pitfalls & Design Decisions

This document explains the type conversion challenges encountered while building the UniFFI
bindings layer in `crates/ffi/src/conversions.rs`, and the reasoning behind each design choice.

## 1. Fixed-size arrays are not supported by UniFFI

**Problem:** The `linear-algebra` crate uses Rust fixed-size arrays extensively — `[[f64; 2]; 2]`
for 2x2 matrices, `[[f64; 3]; 3]` for 3x3 matrices, and `[f64; 2]` / `[f64; 3]` for vectors.
UniFFI has no representation for fixed-size arrays because the underlying C FFI layer and the
target languages (Python, Go) have no direct equivalent of a compile-time-sized array type.

**Solution:** We introduced flat-field Record types:

- `Matrix2x2 { m00, m01, m10, m11 }` replaces `[[f64; 2]; 2]`
- `Matrix3x3 { m00..m22 }` replaces `[[f64; 3]; 3]`
- `Point2 { x, y }` replaces `[f64; 2]`
- `Point3 { x, y, z }` replaces `[f64; 3]`

**Why flat fields instead of `Vec<f64>`?** Flat-field Records provide compile-time guarantees on
the number of elements. A `Vec<f64>` would require runtime length validation at the FFI boundary,
adding error paths that don't exist in the original API. With Records, if a caller constructs a
`Matrix2x2`, it always has exactly 4 elements — the type system enforces this in every target
language.

**Conversion pattern:** Each wrapper function converts at the boundary:
```rust
pub fn determinant_2x2(m: Matrix2x2) -> f64 {
    la::determinant_2x2((&m).into())  // Matrix2x2 → [[f64; 2]; 2]
}
```

## 2. Tuples are not supported by UniFFI

**Problem:** Several functions return Rust tuples — `(f64, f64)` from `quadratic_roots` and
`midpoint_2d`, and `(Complex, Complex)` from `quadratic_roots_complex`. UniFFI cannot serialize
tuples across the FFI boundary because tuples are anonymous product types with no stable field
names, and target languages need named accessors for generated bindings.

**Solution:** Named Record types replace each tuple:

- `F64Pair { first, second }` replaces `(f64, f64)`
- `ComplexPair { first, second }` replaces `(Complex, Complex)`
- `Point2 { x, y }` replaces `(f64, f64)` where the values represent coordinates

**Why separate types for `F64Pair` vs `Point2`?** Although both wrap two `f64` values, they carry
different semantic meaning. `quadratic_roots` returns two roots (first/second), while `midpoint_2d`
returns a spatial coordinate (x/y). Using the same type for both would confuse the generated API.

## 3. Enum tuple variants require struct variants for Go compatibility

**Problem:** The core crate defines `MathError::InvalidInput(String)` as a tuple variant. While
UniFFI's proc-macro system can handle tuple variants, `uniffi-bindgen-go` has better compatibility
with struct variants (named fields). Tuple variant serialization varies across binding generators
and can produce awkward generated code in some languages.

**Solution:** The FFI crate defines its own mirror error enum with a struct variant:
```rust
// Internal (core crate):     InvalidInput(String)
// FFI (our conversion):      InvalidInput { message: String }
```

The `From<core::MathError>` impl maps between them:
```rust
core::MathError::InvalidInput(msg) => MathError::InvalidInput { message: msg }
```

**Why not modify the core crate?** Keeping internal crates free of UniFFI dependencies means they
remain usable as pure Rust libraries. The FFI layer is a separate concern — coupling domain types
to a specific binding framework would make the core crate harder to use in contexts where UniFFI
isn't wanted.

## 4. The `Complex` struct: Record vs Object trade-off

**Problem:** The `complex` crate's `Complex` struct has both public fields (`re`, `im`) and
methods (`magnitude()`, `argument()`, `conjugate()`), plus operator trait implementations
(`Add`, `Sub`, `Mul`, `Div`). UniFFI offers two ways to expose structs:

- **Record:** Fields are directly accessible, type is copied by value, but cannot have methods.
- **Object:** Can have methods, but fields become opaque (need getter functions), the type is
  passed by `Arc<Self>` reference, and Go callers must call `Destroy()` to free memory.

**Solution:** We chose Record + free functions:
```rust
#[derive(uniffi::Record)]
pub struct Complex { pub re: f64, pub im: f64 }

#[uniffi::export]
pub fn complex_magnitude(c: Complex) -> f64 { ... }
```

**Reasoning:**
- Complex numbers are small value types (2 x f64) — copying is cheap, reference counting adds overhead.
- Direct field access (`z.re`, `z.im`) is natural for a math library. With Object, users would need `z.re()` getters.
- No `Destroy()` burden on Go callers — Records are plain Go structs with no associated finalizer.
- Operator overloading (`+`, `-`, `*`, `/`) doesn't cross UniFFI anyway, so methods provide no syntax advantage.
- Free functions like `complex_add(a, b)` are explicit and work identically in all target languages.

## 5. Slice parameters become `Vec<T>` at the FFI boundary

**Problem:** Functions like `polynomial_evaluate(coeffs: &[f64], x: f64)` take slice references.
While UniFFI's proc-macro approach can accept `&[f64]` in some cases, the data is always copied
across the FFI boundary regardless — there is no zero-copy slice sharing between Rust and a
foreign language runtime.

**Solution:** FFI wrappers accept `Vec<f64>` and pass `&v` to the internal function:
```rust
pub fn polynomial_evaluate(coeffs: Vec<f64>, x: f64) -> f64 {
    poly::evaluate(&coeffs, x)
}
```

**Why `Vec` instead of trying to use `&[T]`?** Making the ownership transfer explicit avoids
subtle lifetime issues in the generated scaffolding code. The semantic difference is invisible to
callers in Python or Go — they pass a list/slice either way, and the data is copied at the
boundary in both cases.

## 6. Module naming: `arithmetic.rs` instead of `core.rs`

**Problem:** Naming the module `core.rs` (to match the `core` crate) would shadow Rust's built-in
`core` prelude, causing compilation errors when any code in the FFI crate tries to use `core::`
paths.

**Solution:** The module is named `arithmetic.rs` instead. The exported function name (`add`) is
what matters to FFI consumers — the Rust module name is an internal organizational detail.

## 7. Error conversion uses a separate mirror type, not a re-export

**Problem:** To use `#[derive(uniffi::Error)]` on `MathError`, the type must be defined in a crate
that depends on `uniffi`. Adding `uniffi` as a dependency to the core crate would couple every
crate in the workspace to UniFFI (since they all depend on core).

**Solution:** The FFI crate defines its own `MathError` with `#[derive(uniffi::Error)]` and
implements `From<core::MathError>` to convert at the boundary. Each wrapper function calls
`.map_err(MathError::from)` on results from internal crates.

**Trade-off:** This means two `MathError` definitions exist. However, the `From` impl guarantees
they stay in sync — if a variant is added to core's `MathError`, the `From` impl will fail to
compile until the FFI mirror is updated.
