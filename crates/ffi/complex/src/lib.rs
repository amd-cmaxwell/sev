#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!();

use sev_complex as cx;
use sev_types_ffi::MathError;

#[derive(Debug, Clone, uniffi::Record)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ComplexPair {
    pub first: Complex,
    pub second: Complex,
}

impl From<cx::Complex> for Complex {
    fn from(c: cx::Complex) -> Self {
        Complex { re: c.re, im: c.im }
    }
}

impl From<&Complex> for cx::Complex {
    fn from(c: &Complex) -> Self {
        cx::Complex::new(c.re, c.im)
    }
}

#[uniffi::export]
pub fn complex_new(re: f64, im: f64) -> Complex {
    Complex { re, im }
}

#[uniffi::export]
pub fn complex_from_polar(r: f64, theta: f64) -> Complex {
    cx::Complex::from_polar(r, theta).into()
}

#[uniffi::export]
pub fn complex_magnitude(c: Complex) -> f64 {
    cx::Complex::from(&c).magnitude()
}

#[uniffi::export]
pub fn complex_argument(c: Complex) -> f64 {
    cx::Complex::from(&c).argument()
}

#[uniffi::export]
pub fn complex_conjugate(c: Complex) -> Complex {
    cx::Complex::from(&c).conjugate().into()
}

#[uniffi::export]
pub fn complex_add(a: Complex, b: Complex) -> Complex {
    (cx::Complex::from(&a) + cx::Complex::from(&b)).into()
}

#[uniffi::export]
pub fn complex_sub(a: Complex, b: Complex) -> Complex {
    (cx::Complex::from(&a) - cx::Complex::from(&b)).into()
}

#[uniffi::export]
pub fn complex_mul(a: Complex, b: Complex) -> Complex {
    (cx::Complex::from(&a) * cx::Complex::from(&b)).into()
}

#[uniffi::export]
pub fn complex_div(a: Complex, b: Complex) -> Complex {
    (cx::Complex::from(&a) / cx::Complex::from(&b)).into()
}

#[uniffi::export]
pub fn complex_display(c: Complex) -> String {
    cx::Complex::from(&c).to_string()
}

#[uniffi::export]
pub fn quadratic_roots_complex(a: f64, b: f64, c: f64) -> Result<ComplexPair, MathError> {
    let (r1, r2) = cx::quadratic_roots_complex(a, b, c).map_err(MathError::from)?;
    Ok(ComplexPair {
        first: r1.into(),
        second: r2.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_roundtrip_conversion() {
        let ffi = Complex { re: 3.0, im: 4.0 };
        let lib: cx::Complex = (&ffi).into();
        let back: Complex = lib.into();
        assert_eq!(back.re, ffi.re);
        assert_eq!(back.im, ffi.im);
    }

    #[test]
    fn test_complex_pair_from_quadratic() {
        let pair = quadratic_roots_complex(1.0, 0.0, -1.0).unwrap();
        assert!((pair.first.re - 1.0).abs() < 1e-10 || (pair.second.re - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_error_mapping() {
        let err = quadratic_roots_complex(0.0, 1.0, 1.0).unwrap_err();
        assert!(matches!(err, MathError::InvalidInput { .. }));
    }
}
