#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!();

use sev_sequences as seq;
use sev_types_ffi::MathError;

#[uniffi::export]
pub fn arithmetic_term(a1: f64, d: f64, n: u64) -> f64 {
    seq::arithmetic_term(a1, d, n)
}

#[uniffi::export]
pub fn arithmetic_sum(a1: f64, d: f64, n: u64) -> f64 {
    seq::arithmetic_sum(a1, d, n)
}

#[uniffi::export]
pub fn geometric_term(a1: f64, r: f64, n: u64) -> f64 {
    seq::geometric_term(a1, r, n)
}

#[uniffi::export]
pub fn geometric_sum(a1: f64, r: f64, n: u64) -> f64 {
    seq::geometric_sum(a1, r, n)
}

#[uniffi::export]
pub fn geometric_sum_infinite(a1: f64, r: f64) -> Result<f64, MathError> {
    seq::geometric_sum_infinite(a1, r).map_err(MathError::from)
}

#[uniffi::export]
pub fn factorial(n: u64) -> Result<u64, MathError> {
    seq::factorial(n).map_err(MathError::from)
}

#[uniffi::export]
pub fn permutations(n: u64, r: u64) -> Result<u64, MathError> {
    seq::permutations(n, r).map_err(MathError::from)
}

#[uniffi::export]
pub fn combinations(n: u64, r: u64) -> Result<u64, MathError> {
    seq::combinations(n, r).map_err(MathError::from)
}

#[uniffi::export]
pub fn gcd(a: u64, b: u64) -> u64 {
    seq::gcd(a, b)
}

#[uniffi::export]
pub fn lcm(a: u64, b: u64) -> Result<u64, MathError> {
    seq::lcm(a, b).map_err(MathError::from)
}

#[uniffi::export]
pub fn fibonacci(n: u64) -> Result<u64, MathError> {
    seq::fibonacci(n).map_err(MathError::from)
}

#[uniffi::export]
pub fn is_prime(n: u64) -> bool {
    seq::is_prime(n)
}