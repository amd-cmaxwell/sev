use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    DivisionByZero,
    InvalidInput(String),
    NoRealSolution,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "division by zero"),
            MathError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            MathError::NoRealSolution => write!(f, "no real solution"),
            MathError::Overflow => write!(f, "arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}

pub type Result<T> = std::result::Result<T, MathError>;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_math_error_display() {
        assert_eq!(MathError::DivisionByZero.to_string(), "division by zero");
        assert_eq!(
            MathError::InvalidInput("bad".into()).to_string(),
            "invalid input: bad"
        );
        assert_eq!(MathError::NoRealSolution.to_string(), "no real solution");
        assert_eq!(MathError::Overflow.to_string(), "arithmetic overflow");
    }
}
