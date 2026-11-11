use sev_core as core;

pub fn evaluate(coeffs: &[f64], x: f64) -> f64 {
    coeffs.iter().fold(0.0, |acc, &c| acc * x + c)
}

pub fn quadratic_roots(a: f64, b: f64, c: f64) -> core::Result<(f64, f64)> {
    if a.abs() < 1e-12 {
        return Err(core::MathError::DivisionByZero);
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return Err(core::MathError::NoRealSolution);
    }
    let sqrt_d = discriminant.sqrt();
    let r1 = (-b - sqrt_d) / (2.0 * a);
    let r2 = (-b + sqrt_d) / (2.0 * a);
    if r1 <= r2 {
        Ok((r1, r2))
    } else {
        Ok((r2, r1))
    }
}

pub fn linear_root(m: f64, b: f64) -> core::Result<f64> {
    if m.abs() < 1e-12 {
        return Err(core::MathError::DivisionByZero);
    }
    Ok(-b / m)
}

pub fn add(a: &[f64], b: &[f64]) -> Vec<f64> {
    let len = a.len().max(b.len());
    let mut result = vec![0.0; len];
    let a_offset = len - a.len();
    let b_offset = len - b.len();
    for (i, &v) in a.iter().enumerate() {
        result[a_offset + i] += v;
    }
    for (i, &v) in b.iter().enumerate() {
        result[b_offset + i] += v;
    }
    result
}

pub fn multiply(a: &[f64], b: &[f64]) -> Vec<f64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut result = vec![0.0; a.len() + b.len() - 1];
    for (i, &av) in a.iter().enumerate() {
        for (j, &bv) in b.iter().enumerate() {
            result[i + j] += av * bv;
        }
    }
    result
}

pub fn derivative(coeffs: &[f64]) -> Vec<f64> {
    if coeffs.len() <= 1 {
        return Vec::new();
    }
    let degree = coeffs.len() - 1;
    coeffs[..degree]
        .iter()
        .enumerate()
        .map(|(i, &c)| c * (degree - i) as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let coeffs = [1.0, -3.0, 2.0];
        assert!((evaluate(&coeffs, 1.0) - 0.0).abs() < 1e-12);
        assert!((evaluate(&coeffs, 2.0) - 0.0).abs() < 1e-12);
        assert!((evaluate(&coeffs, 0.0) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_evaluate_empty() {
        assert!((evaluate(&[], 5.0) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_quadratic_roots() {
        let (r1, r2) = quadratic_roots(1.0, -5.0, 6.0).unwrap();
        assert!((r1 - 2.0).abs() < 1e-9);
        assert!((r2 - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_quadratic_roots_no_real() {
        let result = quadratic_roots(1.0, 0.0, 1.0);
        assert_eq!(result.unwrap_err(), core::MathError::NoRealSolution);
    }

    #[test]
    fn test_quadratic_roots_a_zero() {
        let result = quadratic_roots(0.0, 2.0, 1.0);
        assert_eq!(result.unwrap_err(), core::MathError::DivisionByZero);
    }

    #[test]
    fn test_quadratic_roots_double() {
        let (r1, r2) = quadratic_roots(1.0, -4.0, 4.0).unwrap();
        assert!((r1 - 2.0).abs() < 1e-9);
        assert!((r2 - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_linear_root() {
        let r = linear_root(2.0, 4.0).unwrap();
        assert!((r - (-2.0)).abs() < 1e-12);
    }

    #[test]
    fn test_linear_root_zero_slope() {
        let result = linear_root(0.0, 5.0);
        assert_eq!(result.unwrap_err(), core::MathError::DivisionByZero);
    }

    #[test]
    fn test_add() {
        let result = add(&[1.0, 2.0], &[3.0, 4.0, 5.0]);
        assert_eq!(result, vec![3.0, 5.0, 7.0]);
    }

    #[test]
    fn test_add_same_length() {
        let result = add(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_multiply() {
        let result = multiply(&[1.0, 1.0], &[1.0, -1.0]);
        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.0).abs() < 1e-12);
        assert!((result[1] - 0.0).abs() < 1e-12);
        assert!((result[2] - (-1.0)).abs() < 1e-12);
    }

    #[test]
    fn test_multiply_empty() {
        assert!(multiply(&[], &[1.0, 2.0]).is_empty());
        assert!(multiply(&[1.0], &[]).is_empty());
    }

    #[test]
    fn test_derivative() {
        let result = derivative(&[3.0, 2.0, 1.0]);
        assert_eq!(result.len(), 2);
        assert!((result[0] - 6.0).abs() < 1e-12);
        assert!((result[1] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_derivative_constant() {
        assert!(derivative(&[5.0]).is_empty());
    }

    #[test]
    fn test_derivative_empty() {
        assert!(derivative(&[]).is_empty());
    }
}
