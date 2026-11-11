#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!();

use sev_trig as trig_crate;
use sev_types_ffi::{MathError, Point2};

#[uniffi::export]
pub fn degrees_to_radians(deg: f64) -> f64 {
    trig_crate::degrees_to_radians(deg)
}

#[uniffi::export]
pub fn radians_to_degrees(rad: f64) -> f64 {
    trig_crate::radians_to_degrees(rad)
}

#[uniffi::export]
pub fn law_of_cosines_side(a: f64, b: f64, angle_c: f64) -> f64 {
    trig_crate::law_of_cosines_side(a, b, angle_c)
}

#[uniffi::export]
pub fn law_of_cosines_angle(a: f64, b: f64, c: f64) -> Result<f64, MathError> {
    trig_crate::law_of_cosines_angle(a, b, c).map_err(MathError::from)
}

#[uniffi::export]
pub fn law_of_sines_side(a: f64, angle_a: f64, angle_b: f64) -> f64 {
    trig_crate::law_of_sines_side(a, angle_a, angle_b)
}

#[uniffi::export]
pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    trig_crate::distance_2d(x1, y1, x2, y2)
}

#[uniffi::export]
pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    trig_crate::distance_3d(x1, y1, z1, x2, y2, z2)
}

#[uniffi::export]
pub fn midpoint_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> Point2 {
    let (mx, my) = trig_crate::midpoint_2d(x1, y1, x2, y2);
    Point2 { x: mx, y: my }
}

#[uniffi::export]
pub fn triangle_area_heron(a: f64, b: f64, c: f64) -> Result<f64, MathError> {
    trig_crate::triangle_area_heron(a, b, c).map_err(MathError::from)
}

#[uniffi::export]
pub fn circle_area(radius: f64) -> f64 {
    trig_crate::circle_area(radius)
}

#[uniffi::export]
pub fn circle_circumference(radius: f64) -> f64 {
    trig_crate::circle_circumference(radius)
}