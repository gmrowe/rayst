use crate::color::Color;
use crate::matrix::Mat4;
use crate::tup::Tup;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Pattern {
    pattern: Patterns,
    transform: Mat4,
}

impl Pattern {
    pub fn stripe_pattern(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: Patterns::Stripe(StripePattern::new(color_a, color_b)),
            ..Self::default()
        }
    }

    pub fn gradient_pattern(from: Color, to: Color) -> Self {
        Self {
            pattern: Patterns::Gradient(GradientPattern::new(from, to)),
            ..Self::default()
        }
    }

    pub fn ring_pattern(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: Patterns::Ring(RingPattern::new(color_a, color_b)),
            ..Self::default()
        }
    }

    pub fn checkers_pattern(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: Patterns::Checker(CheckersPattern::new(color_a, color_b)),
            ..Self::default()
        }
    }

    pub fn with_transform(self, transform: Mat4) -> Self {
        Self { transform, ..self }
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    pub fn color_at(&self, object_transform: Mat4, point: Tup) -> Color {
        let pattern_point = self.transform.inverse() * object_transform.inverse() * point;
        self.pattern.color(pattern_point)
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            pattern: Patterns::Default(DefaultPattern),
            transform: Mat4::identity_matrix(),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct DefaultPattern;

impl DefaultPattern {
    fn pattern_at(&self, point: Tup) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {
    fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    fn a(&self) -> Color {
        self.color_a
    }

    fn b(&self) -> Color {
        self.color_b
    }

    fn pattern_at(&self, point: Tup) -> Color {
        let lattice_point = point.x.floor().abs() as u64;
        if lattice_point % 2 == 0 {
            self.a()
        } else {
            self.b()
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct GradientPattern {
    from: Color,
    to: Color,
}

impl GradientPattern {
    fn new(from: Color, to: Color) -> Self {
        Self { from, to }
    }

    fn pattern_at(&self, point: Tup) -> Color {
        let distance = self.to - self.from;
        let fraction = point.x - point.x.floor();
        self.from + distance * fraction
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct RingPattern {
    color_a: Color,
    color_b: Color,
}

impl RingPattern {
    fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    fn pattern_at(&self, point: Tup) -> Color {
        let x_square = point.x * point.x;
        let z_square = point.z * point.z;
        let lattice_distance = (x_square + z_square).sqrt().floor() as u64;
        if lattice_distance % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
struct CheckersPattern {
    color_a: Color,
    color_b: Color,
}

impl CheckersPattern {
    fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    fn pattern_at(&self, point: Tup) -> Color {
        let lattice_x = point.x.floor().abs() as u64;
        let lattice_y = point.y.floor().abs() as u64;
        let lattice_z = point.z.floor().abs() as u64;
        if (lattice_x + lattice_y + lattice_z) % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Patterns {
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Default(DefaultPattern),
    Ring(RingPattern),
    Checker(CheckersPattern),
}

impl Patterns {
    fn color(&self, pattern_point: Tup) -> Color {
        match self {
            Patterns::Stripe(p) => p.pattern_at(pattern_point),
            Patterns::Gradient(p) => p.pattern_at(pattern_point),
            Patterns::Default(p) => p.pattern_at(pattern_point),
            Patterns::Ring(p) => p.pattern_at(pattern_point),
            Patterns::Checker(p) => p.pattern_at(pattern_point),
        }
    }
}

#[cfg(test)]
mod patterns_test {
    use super::*;
    use crate::color::consts as color;
    use crate::shapes::Shape;
    use crate::spheres::Sphere;
    use crate::transforms;

    #[test]
    fn a_stripe_pattern_can_be_created() {
        let stripe_pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, stripe_pattern.a());
        assert_eq!(color::BLACK, stripe_pattern.b());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let stripe_pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, stripe_pattern.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(color::WHITE, stripe_pattern.pattern_at(Tup::point(0, 1, 0)));
        assert_eq!(color::WHITE, stripe_pattern.pattern_at(Tup::point(0, 2, 0)));
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let stripe_pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::BLACK, stripe_pattern.pattern_at(Tup::point(1, 0, 0)));
        assert_eq!(color::BLACK, stripe_pattern.pattern_at(Tup::point(1, 0, 1)));
        assert_eq!(color::BLACK, stripe_pattern.pattern_at(Tup::point(1, 0, 2)));
    }

    #[test]
    fn a_stripe_pattern_is_changes_with_x() {
        let stripe_pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, stripe_pattern.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(
            color::WHITE,
            stripe_pattern.pattern_at(Tup::point(0.9, 0.0, 0.0))
        );
        assert_eq!(color::BLACK, stripe_pattern.pattern_at(Tup::point(1, 0, 0)));
        assert_eq!(
            color::BLACK,
            stripe_pattern.pattern_at(Tup::point(-0.1, 0.0, 0.0))
        );
        assert_eq!(
            color::BLACK,
            stripe_pattern.pattern_at(Tup::point(-1, 0, 0))
        );
        assert_eq!(
            color::WHITE,
            stripe_pattern.pattern_at(Tup::point(-1.1, 0.0, 0.0))
        );
    }

    #[test]
    fn a_pattern_can_have_an_object_transformation() {
        let object = Sphere::default().with_transform(transforms::scaling(2, 2, 2));
        let pattern = Pattern::stripe_pattern(color::WHITE, color::BLACK);
        let c = pattern.color_at(object.transform(), Tup::point(1.5, 0.0, 0.0));
        assert_eq!(color::WHITE, c);
    }

    #[test]
    fn a_pattern_can_have_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = Pattern::stripe_pattern(color::WHITE, color::BLACK)
            .with_transform(transforms::scaling(2, 2, 2));
        let c = pattern.color_at(object.transform(), Tup::point(2.5, 0.0, 0.0));
        assert_eq!(color::BLACK, c);
    }

    #[test]
    fn a_pattern_can_have_a_pattern_and_object_transformation() {
        let object = Sphere::default().with_transform(transforms::scaling(2, 2, 2));
        let pattern = Pattern::stripe_pattern(color::WHITE, color::BLACK)
            .with_transform(transforms::translation(0.5, 0.0, 0.0));
        let c = pattern.color_at(object.transform(), Tup::point(2.5, 0.0, 0.0));
        assert_eq!(color::WHITE, c);
    }

    #[test]
    fn a_default_pattern_has_an_identity_transformation() {
        let pattern = Pattern::default();
        assert_eq!(Mat4::identity_matrix(), pattern.transform());
    }

    #[test]
    fn a_transform_can_be_assigned_to_a_pattern() {
        let pattern = Pattern::default().with_transform(transforms::translation(1, 2, 3));
        assert_eq!(transforms::translation(1, 2, 3), pattern.transform());
    }

    #[test]
    fn a_pattern_correctly_handles_an_object_transformation() {
        let object = Sphere::default().with_transform(transforms::scaling(2, 2, 2));
        let pattern = Pattern::default();
        let c = pattern.color_at(object.transform(), Tup::point(2, 3, 4));
        assert_eq!(Color::new(1.0, 1.5, 2.0), c);
    }

    #[test]
    fn a_pattern_correctly_handles_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = Pattern::default().with_transform(transforms::scaling(2, 2, 2));
        let c = pattern.color_at(object.transform(), Tup::point(2, 3, 4));
        assert_eq!(Color::new(1.0, 1.5, 2.0), c);
    }

    #[test]
    fn a_pattern_correctly_handles_a_pattern_plus_object_transformation() {
        let object = Sphere::default().with_transform(transforms::scaling(2, 2, 2));
        let pattern = Pattern::default().with_transform(transforms::translation(0.5, 1.0, 1.5));
        let c = pattern.color_at(object.transform(), Tup::point(2.5, 3.0, 3.5));
        assert_eq!(Color::new(0.75, 0.5, 0.25), c);
    }

    #[test]
    fn a_gradient_pattern_linearly_interpolates_between_colors() {
        let p = GradientPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            p.pattern_at(Tup::point(0.25, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            p.pattern_at(Tup::point(0.5, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            p.pattern_at(Tup::point(0.75, 0.0, 0.0))
        );
    }

    #[test]
    fn a_ring_pattern_extends_in_x_and_z() {
        let p = RingPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(color::BLACK, p.pattern_at(Tup::point(1, 0, 0)));
        assert_eq!(color::BLACK, p.pattern_at(Tup::point(0, 0, 1)));
        assert_eq!(color::BLACK, p.pattern_at(Tup::point(0.708, 0.0, 0.708)));
    }

    #[test]
    fn a_checkers_pattern_repeats_in_x() {
        let p = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0.99, 0.0, 0.0)));
        assert_eq!(color::BLACK, p.pattern_at(Tup::point(1.01, 0.0, 0.0)));
    }

    #[test]
    fn a_checkers_pattern_repeats_in_y() {
        let p = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0.0, 0.99, 0.0)));
        assert_eq!(color::BLACK, p.pattern_at(Tup::point(0.0, 1.01, 0.0)));
    }

    #[test]
    fn a_checkers_pattern_repeats_in_z() {
        let p = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0, 0, 0)));
        assert_eq!(color::WHITE, p.pattern_at(Tup::point(0.0, 0.0, 0.99)));
        assert_eq!(color::BLACK, p.pattern_at(Tup::point(0.0, 0.0, 1.01)));
    }

    
}
