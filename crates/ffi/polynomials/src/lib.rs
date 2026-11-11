#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!();

use sev_polynomials as poly;
use sev_types_ffi::MathError;

#[derive(Debug, Clone, uniffi::Record)]
pub struct F64Pair {
    pub first: f64,
    pub second: f64,
}

#[uniffi::export]
pub fn polynomial_evaluate(coeffs: Vec<f64>, x: f64) -> f64 {
    poly::evaluate(&coeffs, x)
}

#[uniffi::export]
pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Result<F64Pair, MathError> {
    let (r1, r2) = poly::quadratic_roots(a, b, c).map_err(MathError::from)?;
    Ok(F64Pair {
        first: r1,
        second: r2,
    })
}

#[uniffi::export]
pub fn linear_root(m: f64, b: f64) -> Result<f64, MathError> {
    poly::linear_root(m, b).map_err(MathError::from)
}

#[uniffi::export]
pub fn polynomial_add(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    poly::add(&a, &b)
}

#[uniffi::export]
pub fn polynomial_multiply(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    poly::multiply(&a, &b)
}

#[uniffi::export]
pub fn polynomial_derivative(coeffs: Vec<f64>) -> Vec<f64> {
    poly::derivative(&coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64pair_from_quadratic() {
        let roots = quadratic_roots(1.0, -3.0, 2.0).unwrap();
        assert!((roots.first - 2.0).abs() < 1e-10 || (roots.first - 1.0).abs() < 1e-10);
        assert!((roots.second - 2.0).abs() < 1e-10 || (roots.second - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_error_mapping_division_by_zero() {
        let err = linear_root(0.0, 1.0).unwrap_err();
        assert!(matches!(err, MathError::DivisionByZero));
    }
}
