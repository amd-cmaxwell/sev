#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!();

use std::fmt;
use sev_core as core;

#[derive(Debug, Clone, PartialEq, uniffi::Error)]
pub enum MathError {
    DivisionByZero,
    InvalidInput { message: String },
    NoRealSolution,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "division by zero"),
            MathError::InvalidInput { message } => write!(f, "invalid input: {message}"),
            MathError::NoRealSolution => write!(f, "no real solution"),
            MathError::Overflow => write!(f, "arithmetic overflow"),
        }
    }
}

impl From<core::MathError> for MathError {
    fn from(e: core::MathError) -> Self {
        match e {
            core::MathError::DivisionByZero => MathError::DivisionByZero,
            core::MathError::InvalidInput(msg) => MathError::InvalidInput { message: msg },
            core::MathError::NoRealSolution => MathError::NoRealSolution,
            core::MathError::Overflow => MathError::Overflow,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl From<[f64; 2]> for Point2 {
    fn from(v: [f64; 2]) -> Self {
        Point2 { x: v[0], y: v[1] }
    }
}

impl From<&Point2> for [f64; 2] {
    fn from(p: &Point2) -> Self {
        [p.x, p.y]
    }
}
