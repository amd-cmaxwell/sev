use sev_core as core;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn argument(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{} + {}i", self.re, self.im)
        } else {
            write!(f, "{} - {}i", self.re, -self.im)
        }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

pub fn quadratic_roots_complex(a: f64, b: f64, c: f64) -> core::Result<(Complex, Complex)> {
    if a == 0.0 {
        return Err(core::MathError::InvalidInput(
            "coefficient 'a' must be non-zero for a quadratic".into(),
        ));
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant >= 0.0 {
        let sqrt_d = discriminant.sqrt();
        let r1 = Complex::new((-b + sqrt_d) / (2.0 * a), 0.0);
        let r2 = Complex::new((-b - sqrt_d) / (2.0 * a), 0.0);
        Ok((r1, r2))
    } else {
        let real_part = -b / (2.0 * a);
        let imag_part = (-discriminant).sqrt() / (2.0 * a);
        let r1 = Complex::new(real_part, imag_part);
        let r2 = Complex::new(real_part, -imag_part);
        Ok((r1, r2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn complex_approx_eq(a: Complex, b: Complex) -> bool {
        approx_eq(a.re, b.re) && approx_eq(a.im, b.im)
    }

    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a + b;
        assert!(complex_approx_eq(c, Complex::new(4.0, 6.0)));
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(5.0, 3.0);
        let b = Complex::new(2.0, 1.0);
        let c = a - b;
        assert!(complex_approx_eq(c, Complex::new(3.0, 2.0)));
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        // (1+2i)(3+4i) = 3+4i+6i+8i² = 3+10i-8 = -5+10i
        let c = a * b;
        assert!(complex_approx_eq(c, Complex::new(-5.0, 10.0)));
    }

    #[test]
    fn test_div() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a / b;
        // (1+2i)/(3+4i) = (1+2i)(3-4i)/25 = (3-4i+6i-8i²)/25 = (11+2i)/25
        assert!(complex_approx_eq(c, Complex::new(11.0 / 25.0, 2.0 / 25.0)));
    }

    #[test]
    fn test_magnitude() {
        let z = Complex::new(3.0, 4.0);
        assert!(approx_eq(z.magnitude(), 5.0));
    }

    #[test]
    fn test_from_polar_roundtrip() {
        let r = 5.0;
        let theta = std::f64::consts::FRAC_PI_4;
        let z = Complex::from_polar(r, theta);
        assert!(approx_eq(z.magnitude(), r));
        assert!(approx_eq(z.argument(), theta));
    }

    #[test]
    fn test_conjugate() {
        let z = Complex::new(3.0, 4.0);
        let conj = z.conjugate();
        assert!(complex_approx_eq(conj, Complex::new(3.0, -4.0)));
        // z * conj(z) should equal |z|²
        let product = z * conj;
        assert!(approx_eq(product.re, 25.0));
        assert!(approx_eq(product.im, 0.0));
    }

    #[test]
    fn test_display_positive_im() {
        let z = Complex::new(1.0, 2.0);
        assert_eq!(format!("{z}"), "1 + 2i");
    }

    #[test]
    fn test_display_negative_im() {
        let z = Complex::new(1.0, -2.0);
        assert_eq!(format!("{z}"), "1 - 2i");
    }

    #[test]
    fn test_quadratic_roots_positive_discriminant() {
        // x² - 5x + 6 = 0 → x = 3, x = 2
        let (r1, r2) = quadratic_roots_complex(1.0, -5.0, 6.0).unwrap();
        assert!(approx_eq(r1.re, 3.0));
        assert!(approx_eq(r1.im, 0.0));
        assert!(approx_eq(r2.re, 2.0));
        assert!(approx_eq(r2.im, 0.0));
    }

    #[test]
    fn test_quadratic_roots_negative_discriminant() {
        // x² + 1 = 0 → x = ±i
        let (r1, r2) = quadratic_roots_complex(1.0, 0.0, 1.0).unwrap();
        assert!(approx_eq(r1.re, 0.0));
        assert!(approx_eq(r1.im, 1.0));
        assert!(approx_eq(r2.re, 0.0));
        assert!(approx_eq(r2.im, -1.0));
    }

    #[test]
    fn test_quadratic_roots_zero_discriminant() {
        // x² - 2x + 1 = 0 → x = 1 (double root)
        let (r1, r2) = quadratic_roots_complex(1.0, -2.0, 1.0).unwrap();
        assert!(approx_eq(r1.re, 1.0));
        assert!(approx_eq(r2.re, 1.0));
    }

    #[test]
    fn test_quadratic_roots_a_zero() {
        let result = quadratic_roots_complex(0.0, 1.0, 2.0);
        assert!(result.is_err());
    }
}
