use crate::math_helpers::nearly_eq;

pub fn assert_nearly_eq(a: f64, b: f64) {
    assert!(nearly_eq(a, b));
}
