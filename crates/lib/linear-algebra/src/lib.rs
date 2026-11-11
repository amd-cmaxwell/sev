use sev_core as core;

const EPSILON: f64 = 1e-12;

pub fn determinant_2x2(m: [[f64; 2]; 2]) -> f64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

pub fn determinant_3x3(m: [[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

pub fn solve_2x2(a: [[f64; 2]; 2], b: [f64; 2]) -> core::Result<[f64; 2]> {
    let det = determinant_2x2(a);
    if det.abs() < EPSILON {
        return Err(core::MathError::DivisionByZero);
    }
    let x = (b[0] * a[1][1] - b[1] * a[0][1]) / det;
    let y = (a[0][0] * b[1] - a[1][0] * b[0]) / det;
    Ok([x, y])
}

pub fn solve_3x3(a: [[f64; 3]; 3], b: [f64; 3]) -> core::Result<[f64; 3]> {
    let det = determinant_3x3(a);
    if det.abs() < EPSILON {
        return Err(core::MathError::DivisionByZero);
    }

    let dx = determinant_3x3([
        [b[0], a[0][1], a[0][2]],
        [b[1], a[1][1], a[1][2]],
        [b[2], a[2][1], a[2][2]],
    ]);
    let dy = determinant_3x3([
        [a[0][0], b[0], a[0][2]],
        [a[1][0], b[1], a[1][2]],
        [a[2][0], b[2], a[2][2]],
    ]);
    let dz = determinant_3x3([
        [a[0][0], a[0][1], b[0]],
        [a[1][0], a[1][1], b[1]],
        [a[2][0], a[2][1], b[2]],
    ]);

    Ok([dx / det, dy / det, dz / det])
}

pub fn matrix_multiply_2x2(a: [[f64; 2]; 2], b: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

pub fn matrix_inverse_2x2(m: [[f64; 2]; 2]) -> core::Result<[[f64; 2]; 2]> {
    let det = determinant_2x2(m);
    if det.abs() < EPSILON {
        return Err(core::MathError::DivisionByZero);
    }
    let inv_det = 1.0 / det;
    Ok([
        [m[1][1] * inv_det, -m[0][1] * inv_det],
        [-m[1][0] * inv_det, m[0][0] * inv_det],
    ])
}

pub fn transpose_2x2(m: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [[m[0][0], m[1][0]], [m[0][1], m[1][1]]]
}

pub fn dot_product(a: &[f64], b: &[f64]) -> core::Result<f64> {
    if a.len() != b.len() {
        return Err(core::MathError::InvalidInput(
            "vectors must have the same length".into(),
        ));
    }
    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}

pub fn cross_product_3d(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TOL
    }

    #[test]
    fn test_determinant_2x2() {
        assert!(approx_eq(determinant_2x2([[1.0, 0.0], [0.0, 1.0]]), 1.0));
        assert!(approx_eq(determinant_2x2([[2.0, 3.0], [1.0, 4.0]]), 5.0));
        assert!(approx_eq(determinant_2x2([[1.0, 2.0], [2.0, 4.0]]), 0.0));
    }

    #[test]
    fn test_determinant_3x3() {
        let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert!(approx_eq(determinant_3x3(identity), 1.0));

        let m = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        assert!(approx_eq(determinant_3x3(m), 0.0));

        let m2 = [[6.0, 1.0, 1.0], [4.0, -2.0, 5.0], [2.0, 8.0, 7.0]];
        assert!(approx_eq(determinant_3x3(m2), -306.0));
    }

    #[test]
    fn test_solve_2x2() {
        // x + y = 3, 2x - y = 0 => x=1, y=2
        let result = solve_2x2([[1.0, 1.0], [2.0, -1.0]], [3.0, 0.0]).unwrap();
        assert!(approx_eq(result[0], 1.0));
        assert!(approx_eq(result[1], 2.0));
    }

    #[test]
    fn test_solve_2x2_singular() {
        let result = solve_2x2([[1.0, 2.0], [2.0, 4.0]], [1.0, 2.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_3x3() {
        // x + y + z = 6, 2y + 5z = -4, 2x + 5y - z = 27
        let a = [[1.0, 1.0, 1.0], [0.0, 2.0, 5.0], [2.0, 5.0, -1.0]];
        let b = [6.0, -4.0, 27.0];
        let result = solve_3x3(a, b).unwrap();
        assert!(approx_eq(result[0], 5.0));
        assert!(approx_eq(result[1], 3.0));
        assert!(approx_eq(result[2], -2.0));
    }

    #[test]
    fn test_solve_3x3_singular() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b = [1.0, 2.0, 3.0];
        assert!(solve_3x3(a, b).is_err());
    }

    #[test]
    fn test_matrix_multiply_2x2_identity() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let m = [[3.0, 7.0], [2.0, 5.0]];
        let result = matrix_multiply_2x2(m, identity);
        assert!(approx_eq(result[0][0], 3.0));
        assert!(approx_eq(result[0][1], 7.0));
        assert!(approx_eq(result[1][0], 2.0));
        assert!(approx_eq(result[1][1], 5.0));
    }

    #[test]
    fn test_matrix_multiply_2x2() {
        let a = [[1.0, 2.0], [3.0, 4.0]];
        let b = [[5.0, 6.0], [7.0, 8.0]];
        let result = matrix_multiply_2x2(a, b);
        assert!(approx_eq(result[0][0], 19.0));
        assert!(approx_eq(result[0][1], 22.0));
        assert!(approx_eq(result[1][0], 43.0));
        assert!(approx_eq(result[1][1], 50.0));
    }

    #[test]
    fn test_matrix_inverse_2x2_identity() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let inv = matrix_inverse_2x2(identity).unwrap();
        assert!(approx_eq(inv[0][0], 1.0));
        assert!(approx_eq(inv[0][1], 0.0));
        assert!(approx_eq(inv[1][0], 0.0));
        assert!(approx_eq(inv[1][1], 1.0));
    }

    #[test]
    fn test_matrix_inverse_2x2_roundtrip() {
        let m = [[4.0, 7.0], [2.0, 6.0]];
        let inv = matrix_inverse_2x2(m).unwrap();
        let product = matrix_multiply_2x2(m, inv);
        assert!(approx_eq(product[0][0], 1.0));
        assert!(approx_eq(product[0][1], 0.0));
        assert!(approx_eq(product[1][0], 0.0));
        assert!(approx_eq(product[1][1], 1.0));
    }

    #[test]
    fn test_matrix_inverse_2x2_singular() {
        assert!(matrix_inverse_2x2([[1.0, 2.0], [2.0, 4.0]]).is_err());
    }

    #[test]
    fn test_transpose_2x2() {
        let m = [[1.0, 2.0], [3.0, 4.0]];
        let t = transpose_2x2(m);
        assert!(approx_eq(t[0][0], 1.0));
        assert!(approx_eq(t[0][1], 3.0));
        assert!(approx_eq(t[1][0], 2.0));
        assert!(approx_eq(t[1][1], 4.0));
    }

    #[test]
    fn test_dot_product() {
        assert!(approx_eq(dot_product(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).unwrap(), 32.0));
        assert!(approx_eq(dot_product(&[1.0, 0.0], &[0.0, 1.0]).unwrap(), 0.0));
    }

    #[test]
    fn test_dot_product_mismatched() {
        assert!(dot_product(&[1.0, 2.0], &[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn test_cross_product_3d() {
        // i × j = k
        let result = cross_product_3d([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(approx_eq(result[0], 0.0));
        assert!(approx_eq(result[1], 0.0));
        assert!(approx_eq(result[2], 1.0));
    }

    #[test]
    fn test_cross_product_perpendicular() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let c = cross_product_3d(a, b);
        assert!(approx_eq(dot_product(&a, &c).unwrap(), 0.0));
        assert!(approx_eq(dot_product(&b, &c).unwrap(), 0.0));
    }
}
