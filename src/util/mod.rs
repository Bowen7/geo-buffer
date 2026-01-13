//! This module provides a conceptual structure of points and half-lines.
//!
//! See more details on each item.

mod coordinate;
mod ray;

pub use coordinate::Coordinate;
pub use ray::Ray;

const EPS: f32 = 1e-6;

pub(crate) fn feq(x: f32, y: f32) -> bool {
    f32::abs(x - y) < EPS
}

pub(crate) fn fneq(x: f32, y: f32) -> bool {
    !feq(x, y)
}

pub(crate) fn fgt(x: f32, y: f32) -> bool {
    if feq(x, y) {
        return false;
    }
    x > y
}

#[allow(dead_code)]
pub(crate) fn flt(x: f32, y: f32) -> bool {
    if feq(x, y) {
        return false;
    }
    x < y
}

pub(crate) fn fgeq(x: f32, y: f32) -> bool {
    if feq(x, y) {
        return true;
    }
    x > y
}

pub(crate) fn fleq(x: f32, y: f32) -> bool {
    if feq(x, y) {
        return true;
    }
    x < y
}
