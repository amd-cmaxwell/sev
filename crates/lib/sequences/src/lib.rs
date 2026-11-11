use sev_core as core;

pub fn arithmetic_term(a1: f64, d: f64, n: u64) -> f64 {
    a1 + (n as f64 - 1.0) * d
}

pub fn arithmetic_sum(a1: f64, d: f64, n: u64) -> f64 {
    let n_f = n as f64;
    n_f / 2.0 * (2.0 * a1 + (n_f - 1.0) * d)
}

pub fn geometric_term(a1: f64, r: f64, n: u64) -> f64 {
    a1 * r.powi((n as i32) - 1)
}

pub fn geometric_sum(a1: f64, r: f64, n: u64) -> f64 {
    if (r - 1.0).abs() < f64::EPSILON {
        n as f64 * a1
    } else {
        a1 * (1.0 - r.powi(n as i32)) / (1.0 - r)
    }
}

pub fn geometric_sum_infinite(a1: f64, r: f64) -> core::Result<f64> {
    if r.abs() >= 1.0 {
        return Err(core::MathError::InvalidInput(
            "common ratio must satisfy |r| < 1".into(),
        ));
    }
    Ok(a1 / (1.0 - r))
}

pub fn factorial(n: u64) -> core::Result<u64> {
    let mut result: u64 = 1;
    for i in 2..=n {
        result = result.checked_mul(i).ok_or(core::MathError::Overflow)?;
    }
    Ok(result)
}

pub fn permutations(n: u64, r: u64) -> core::Result<u64> {
    if r > n {
        return Err(core::MathError::InvalidInput(
            "r must be <= n".into(),
        ));
    }
    let mut result: u64 = 1;
    for i in (n - r + 1)..=n {
        result = result.checked_mul(i).ok_or(core::MathError::Overflow)?;
    }
    Ok(result)
}

pub fn combinations(n: u64, r: u64) -> core::Result<u64> {
    if r > n {
        return Err(core::MathError::InvalidInput(
            "r must be <= n".into(),
        ));
    }
    let r = r.min(n - r);
    let mut result: u64 = 1;
    for i in 0..r {
        result = result
            .checked_mul(n - i)
            .ok_or(core::MathError::Overflow)?;
        result /= i + 1;
    }
    Ok(result)
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> core::Result<u64> {
    if a == 0 || b == 0 {
        return Ok(0);
    }
    (a / gcd(a, b))
        .checked_mul(b)
        .ok_or(core::MathError::Overflow)
}

pub fn fibonacci(n: u64) -> core::Result<u64> {
    if n == 0 {
        return Ok(0);
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let next = a.checked_add(b).ok_or(core::MathError::Overflow)?;
        a = b;
        b = next;
    }
    Ok(b)
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_term() {
        assert!((arithmetic_term(2.0, 3.0, 5) - 14.0).abs() < f64::EPSILON);
        assert!((arithmetic_term(1.0, 0.0, 10) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_arithmetic_sum() {
        assert!((arithmetic_sum(1.0, 1.0, 100) - 5050.0).abs() < f64::EPSILON);
        assert!((arithmetic_sum(5.0, 0.0, 4) - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_geometric_term() {
        assert!((geometric_term(3.0, 2.0, 4) - 24.0).abs() < f64::EPSILON);
        assert!((geometric_term(1.0, 1.0, 100) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_geometric_sum() {
        assert!((geometric_sum(1.0, 2.0, 4) - 15.0).abs() < f64::EPSILON);
        assert!((geometric_sum(5.0, 1.0, 3) - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_geometric_sum_infinite() {
        assert!((geometric_sum_infinite(1.0, 0.5).unwrap() - 2.0).abs() < f64::EPSILON);
        assert!((geometric_sum_infinite(4.0, 0.5).unwrap() - 8.0).abs() < f64::EPSILON);
        assert!(geometric_sum_infinite(1.0, 1.0).is_err());
        assert!(geometric_sum_infinite(1.0, -1.0).is_err());
        assert!(geometric_sum_infinite(1.0, 2.0).is_err());
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(1).unwrap(), 1);
        assert_eq!(factorial(5).unwrap(), 120);
        assert_eq!(factorial(20).unwrap(), 2_432_902_008_176_640_000);
        assert!(factorial(21).is_err());
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(5, 2).unwrap(), 20);
        assert_eq!(permutations(5, 0).unwrap(), 1);
        assert_eq!(permutations(5, 5).unwrap(), 120);
        assert!(permutations(2, 5).is_err());
    }

    #[test]
    fn test_combinations() {
        assert_eq!(combinations(5, 2).unwrap(), 10);
        assert_eq!(combinations(5, 0).unwrap(), 1);
        assert_eq!(combinations(5, 5).unwrap(), 1);
        assert_eq!(combinations(10, 3).unwrap(), 120);
        assert!(combinations(2, 5).is_err());
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(13, 13), 13);
        assert_eq!(gcd(17, 5), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6).unwrap(), 12);
        assert_eq!(lcm(0, 5).unwrap(), 0);
        assert_eq!(lcm(7, 7).unwrap(), 7);
        assert_eq!(lcm(3, 5).unwrap(), 15);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0).unwrap(), 0);
        assert_eq!(fibonacci(1).unwrap(), 1);
        assert_eq!(fibonacci(2).unwrap(), 1);
        assert_eq!(fibonacci(10).unwrap(), 55);
        assert_eq!(fibonacci(20).unwrap(), 6765);
        assert!(fibonacci(94).is_err());
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(17));
        assert!(!is_prime(18));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }
}
