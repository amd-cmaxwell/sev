# Go Sub-Package Modules for UniFFI Bindings

## Context

The Python bindings have `bindings/python/scripts/generate_modules.py` which reads the generated `sev_ffi.py`, classifies exports by prefix/name into logical modules (core, complex, polynomials, etc.), and generates submodule files. The Go bindings originally had everything in a single flat package `sev_ffi`. This document describes the analogous script that generates Go sub-packages with idiomatic naming (prefix-stripped).

## Approach

`bindings/go/scripts/generate_modules.py` is a Python script that:

1. **Parses** the generated `sev_ffi.go` file using regex to extract exported functions (with full signatures) and exported domain types
2. **Classifies** each export into modules using the same grouping logic as the Python script (adapted for PascalCase)
3. **Generates** Go sub-packages under `bindings/go/` — one directory per module, each containing a `.go` file with:
   - Type aliases for domain types: `type Complex = sev_ffi.Complex`
   - Thin wrapper functions that delegate to the base package with prefix-stripped names: `func New(re, im float64) Complex { return sev_ffi.ComplexNew(re, im) }`

## Module Classification (PascalCase)

| Module | Prefixes | Explicit Names | Types |
|--------|----------|----------------|-------|
| `core` | — | `Add` | — |
| `complexnums` | `Complex` | `QuadraticRootsComplex` | `Complex`, `ComplexPair` |
| `polynomials` | `Polynomial` | `QuadraticRoots`, `LinearRoot` | `F64Pair` |
| `linearalgebra` | `Determinant`, `Solve`, `Matrix`, `Dot`, `Cross`, `Transpose` | — | `Matrix2x2`, `Matrix3x3`, `Point2`, `Point3` |
| `trig` | `Degrees`, `Radians`, `LawOf`, `Distance`, `Midpoint`, `Triangle`, `Circle` | — | — |
| `sequences` | `Arithmetic`, `Geometric` | `Factorial`, `Fibonacci`, `Permutations`, `Combinations`, `Gcd`, `Lcm`, `IsPrime` | — |

**Note**: `complex` is a Go builtin type, so the package is named `complexnums`.

Shared types (`MathError` and its variants/sentinels) are re-exported in every sub-package that has functions returning `error`.

## Prefix Stripping Rules

- Strip the module's PascalCase prefix from function names: `ComplexNew` → `New`, `ComplexAdd` → `Add`
- If stripping would produce a name starting with a digit, keep the original: `Distance2d` stays as `Distance2d`
- If stripping would cause a collision (e.g., `ArithmeticSum` and `GeometricSum` both → `Sum`), all colliding names revert to their originals
- For explicitly-named functions without a matching prefix, keep as-is: `QuadraticRootsComplex` → `QuadraticRootsComplex`
- Types keep their original names (they don't have prefixes)
- Types referenced in function signatures but not owned by the module are automatically aliased

## Generated File Structure

```
bindings/go/
├── scripts/
│   └── generate_modules.py
├── core/
│   └── core.go
├── complexnums/
│   └── complexnums.go
├── polynomials/
│   └── polynomials.go
├── linearalgebra/
│   └── linearalgebra.go
├── trig/
│   └── trig.go
└── sequences/
    └── sequences.go
```

## Usage

### Running the script

```bash
python3 bindings/go/scripts/generate_modules.py bindings/go/sev_ffi/sev_ffi.go
```

Or via the Makefile (runs automatically as part of `generate-go`):

```bash
make generate-go
```

### Importing sub-packages

```go
import (
    "github.com/<owner>/<repo>/bindings/go/complexnums"
    "github.com/<owner>/<repo>/bindings/go/linearalgebra"
)

z := complexnums.New(3.0, 4.0)
fmt.Println(complexnums.Magnitude(z))

det := linearalgebra.Determinant2x2(linearalgebra.Matrix2x2{M00: 2, M01: 3, M10: 1, M11: 4})
```

The flat `sev_ffi` package remains available for direct access to all functions without sub-package organization.
