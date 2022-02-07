use crate::math_helpers::nearly_eq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }

    pub fn to_byte_triple(self) -> (u8, u8, u8) {
        const MAX_SUBPIXEL_VALUE: f64 = 255.0;
        let normalize =
            |subpixel: f64| (subpixel.clamp(0.0, 1.0) * MAX_SUBPIXEL_VALUE).round() as u8;
        (
            normalize(self.red()),
            normalize(self.green()),
            normalize(self.blue()),
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color::new(
            self.red() + other.red(),
            self.green() + other.green(),
            self.blue() + other.blue(),
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.red() - other.red(),
            self.green() - other.green(),
            self.blue() - other.blue(),
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        nearly_eq(self.red(), other.red())
            && nearly_eq(self.green(), other.green())
            && nearly_eq(self.blue(), other.blue())
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(
            self.red() * scalar,
            self.green() * scalar,
            self.blue() * scalar,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self::Output {
        Self::new(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue(),
        )
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    fn assert_nearly_eq(a: f64, b: f64) {
        assert!((a - b).abs() < f64::EPSILON);
    }

    #[test]
    fn colors_have_a_red_component() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_nearly_eq(-0.5, color.red())
    }

    #[test]
    fn colors_have_a_green_component() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_nearly_eq(0.4, color.green())
    }

    #[test]
    fn colors_have_a_blue_component() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_nearly_eq(1.7, color.blue())
    }

    #[test]
    fn colors_can_be_added() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);
        assert_eq!(expected, c1 + c2);
    }

    #[test]
    fn colors_can_be_subtracted() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);
        assert_eq!(expected, c1 - c2);
    }

    #[test]
    fn colors_can_be_multiplied_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);
        assert_eq!(expected, c * 2.0);
    }

    #[test]
    fn colors_can_be_multiplied_by_a_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);
        assert_eq!(expected, c1 * c2);
    }
}
