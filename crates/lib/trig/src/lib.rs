use sev_core as core;
use std::f64::consts::PI;

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn radians_to_degrees(rad: f64) -> f64 {
    rad * 180.0 / PI
}

pub fn law_of_cosines_side(a: f64, b: f64, angle_c: f64) -> f64 {
    (a * a + b * b - 2.0 * a * b * angle_c.cos()).sqrt()
}

pub fn law_of_cosines_angle(a: f64, b: f64, c: f64) -> core::Result<f64> {
    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        return Err(core::MathError::InvalidInput(
            "sides must be positive".into(),
        ));
    }
    if a + b <= c || a + c <= b || b + c <= a {
        return Err(core::MathError::InvalidInput(
            "sides do not form a valid triangle".into(),
        ));
    }
    let cos_c = (a * a + b * b - c * c) / (2.0 * a * b);
    Ok(cos_c.acos())
}

pub fn law_of_sines_side(a: f64, angle_a: f64, angle_b: f64) -> f64 {
    a * angle_b.sin() / angle_a.sin()
}

pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

pub fn midpoint_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    ((x1 + x2) / 2.0, (y1 + y2) / 2.0)
}

pub fn triangle_area_heron(a: f64, b: f64, c: f64) -> core::Result<f64> {
    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        return Err(core::MathError::InvalidInput(
            "sides must be positive".into(),
        ));
    }
    let s = (a + b + c) / 2.0;
    let product = s * (s - a) * (s - b) * (s - c);
    if product < 0.0 {
        return Err(core::MathError::InvalidInput(
            "sides do not form a valid triangle".into(),
        ));
    }
    Ok(product.sqrt())
}

pub fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

pub fn circle_circumference(radius: f64) -> f64 {
    2.0 * PI * radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        assert!((degrees_to_radians(90.0) - PI / 2.0).abs() < 1e-10);
        assert!((degrees_to_radians(180.0) - PI).abs() < 1e-10);
        assert!((degrees_to_radians(0.0)).abs() < 1e-10);
        assert!((degrees_to_radians(360.0) - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_radians_to_degrees() {
        assert!((radians_to_degrees(PI / 2.0) - 90.0).abs() < 1e-10);
        assert!((radians_to_degrees(PI) - 180.0).abs() < 1e-10);
        assert!((radians_to_degrees(0.0)).abs() < 1e-10);
    }

    #[test]
    fn test_law_of_cosines_side_right_triangle() {
        // 3-4-5 right triangle: angle C = π/2, sides a=3, b=4
        let c = law_of_cosines_side(3.0, 4.0, PI / 2.0);
        assert!((c - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_law_of_cosines_angle_right_triangle() {
        // 3-4-5: angle opposite side 5 should be π/2
        let angle = law_of_cosines_angle(3.0, 4.0, 5.0).unwrap();
        assert!((angle - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_law_of_cosines_angle_invalid_triangle() {
        assert!(law_of_cosines_angle(1.0, 2.0, 10.0).is_err());
        assert!(law_of_cosines_angle(-1.0, 2.0, 3.0).is_err());
        assert!(law_of_cosines_angle(0.0, 2.0, 3.0).is_err());
    }

    #[test]
    fn test_law_of_sines_side() {
        // Equilateral triangle: all angles 60°, all sides equal
        let angle = PI / 3.0;
        let b = law_of_sines_side(5.0, angle, angle);
        assert!((b - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_distance_2d() {
        assert!((distance_2d(0.0, 0.0, 3.0, 4.0) - 5.0).abs() < 1e-10);
        assert!((distance_2d(1.0, 1.0, 1.0, 1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_distance_3d() {
        assert!((distance_3d(0.0, 0.0, 0.0, 1.0, 2.0, 2.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_midpoint_2d() {
        let (mx, my) = midpoint_2d(0.0, 0.0, 4.0, 6.0);
        assert!((mx - 2.0).abs() < 1e-10);
        assert!((my - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area_heron() {
        // 3-4-5 right triangle: area = 6
        let area = triangle_area_heron(3.0, 4.0, 5.0).unwrap();
        assert!((area - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area_heron_equilateral() {
        // Equilateral triangle with side 2: area = sqrt(3)
        let area = triangle_area_heron(2.0, 2.0, 2.0).unwrap();
        assert!((area - 3.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area_heron_invalid() {
        assert!(triangle_area_heron(1.0, 2.0, 10.0).is_err());
        assert!(triangle_area_heron(-1.0, 2.0, 3.0).is_err());
    }

    #[test]
    fn test_circle_area() {
        assert!((circle_area(1.0) - PI).abs() < 1e-10);
        assert!((circle_area(3.0) - 9.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_circle_circumference() {
        assert!((circle_circumference(1.0) - 2.0 * PI).abs() < 1e-10);
        assert!((circle_circumference(5.0) - 10.0 * PI).abs() < 1e-10);
    }
}
