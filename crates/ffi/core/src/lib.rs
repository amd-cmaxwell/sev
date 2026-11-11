#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn add(a: i32, b: i32) -> i32 {
    sev_core::add(a, b)
}