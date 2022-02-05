use std::ops::{Add, Sub, Neg, Mul, Div};

fn main() {
    use std::iter::successors;
    let projectile = Projectile::new(
        Tup::point(0.0, 1.0, 0.0),
        Tup::vector(1.0, 1.0, 0.0).normalize()
    );

    let environment = Environment::new(
        Tup::vector(0.0, -0.1, 0.0),
        Tup::vector(-0.02, 0.01, 0.0),
    );

    let final_pos =
        successors(Some(projectile), |p| Some(environment.tick(p)))
        .take_while(|p| p.pos.y > 0.0)
        .last()
        .expect("No y values > 0")
        .pos;

    println!("Final position: {:?}", final_pos);
}

struct Projectile {
    pos: Tup, // Point
    vel: Tup, // Vector
}

impl Projectile {
    fn new(pos: Tup, vel: Tup) -> Self {
        assert!(pos.is_point(), "Position must be a point");
        assert!(vel.is_vector(), "Velocity must be a vector");
        Self {
            pos, vel
        }
    }
}

struct Environment {
    gravity: Tup, // Vector
    wind: Tup,    // Vector 
}

impl Environment {
    fn new(gravity: Tup, wind: Tup) -> Self {
        assert!(gravity.is_vector(), "Gravity must be a vector");
        assert!(wind.is_vector(), "Wind must be a vector");
        Self {
            gravity, wind
        }
    }
    
    fn tick(&self, proj: &Projectile) -> Projectile {
        let new_pos = proj.pos + proj.vel;
        let new_vel = proj.vel + self.gravity + self.wind;
        Projectile::new(new_pos, new_vel)
    }
}

#[derive(Clone, Copy, Debug)]
struct Tup {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tup {
    const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0};
    
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x, y, z, w
        }
    }

    fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    fn is_point(&self) -> bool {
        nearly_eq(self.w, 1.0)
    }

    fn is_vector(&self) -> bool {
        nearly_eq(self.w, 0.0)
    }

    fn magnitude(self) -> f64 {
        let sum_of_squares =
            self.x.powf(2.0)
            + self.y.powf(2.0)
            + self.z.powf(2.0)
            + self.w.powf(2.0);

        sum_of_squares.sqrt()
    }

    fn normalize(self) -> Self {
        self / self.magnitude()
    }

    fn dot(self, other: Self) -> f64 {
        self.x * other.x +
            self.y * other.y +
            self.z * other.z +
            self.w * other.w
    }

    fn cross(self, other: Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        nearly_eq(self.x, other.x)
            && nearly_eq(self.y, other.y)
            && nearly_eq(self.z, other.z)
            && nearly_eq(self.w, other.w)
    }
}

impl Add for Tup {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w
        )
    }
}


impl Sub for Tup {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w
        )
    }
}

impl Neg for Tup {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::ZERO - self
    }
}

impl Mul<f64> for Tup {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar
        )
    }
}

impl Div<f64> for Tup {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        assert!(!nearly_eq(scalar, 0.0), "Cannot divide by zero");
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar
        )
    }
}

fn nearly_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

#[cfg(test)]
mod tup_tests {
    use super::*;

    fn assert_nearly_eq(a: f64, b: f64) {
        assert!(nearly_eq(a, b));
    }

    #[test]
    fn a_tup_stores_its_x_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert_nearly_eq(tup.x, 4.3);
    }

    #[test]
    fn a_tup_stores_ites_y_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert_nearly_eq(tup.y, -4.2);
    }

    #[test]
    fn a_tup_stores_ites_z_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert_nearly_eq(tup.z, 3.1);
    }

    #[test]
    fn a_tup_stores_ites_w_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert_nearly_eq(tup.w, 1.0);
    }

    #[test]
    fn a_tup_where_w_equals_1_is_a_point() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert!(tup.is_point());
    }

    #[test]
    fn a_tup_where_w_equals_1_is_not_vector() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert!(!tup.is_vector());
    }

    #[test]
    fn a_tup_where_w_equals_0_is_not_a_point() {
        let tup = Tup::new(4.3, -4.2, 3.1, 0.0);
        assert!(!tup.is_point());
    }

    #[test]
    fn a_tup_where_w_equals_0_is_a_vector() {
        let tup = Tup::new(4.3, -4.2, 3.1, 0.0);
        assert!(tup.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_equals_1() {
        let pt = Tup::point(4.0, -4.0, 3.0);
        assert_eq!(pt, Tup::new(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn vector_creates_tuples_with_w_equals_0() {
        let vx = Tup::vector(4.0, -4.0, 3.0);
        assert_eq!(vx, Tup::new(4.0, -4.0, 3.0, 0.0))
    }

    #[test]
    fn same_vectors_are_equal() {
        let v1 = Tup::vector(1.0, -2.0, 3.5);
        let v2 = Tup::vector(1.0, -2.0, 3.5);
        assert!(v1 == v2);
    }

    #[test]
    fn same_points_are_equal() {
        let v1 = Tup::point(1.0, -2.0, 3.5);
        let v2 = Tup::point(1.0, -2.0, 3.5);
        assert!(v1 == v2);
    }    

    #[test]
    fn vectors_and_points_are_not_equal() {
        let v1 = Tup::vector(1.0, -2.0, 3.5);
        let v2 = Tup::point(1.0, -2.0, 3.5);
        assert!(v1 != v2);
    }

    #[test]
    fn tups_can_be_added() {
        let a1 = Tup::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tup::new(-2.0, 3.0, 1.0, 0.0);
        let expected = Tup::new(1.0, 1.0, 6.0, 1.0);
        assert_eq!(expected, a1 + a2);
    }

    #[test]
    fn adding_two_vectors_produces_a_vector() {
        let a1 = Tup::vector(3.0, -2.0, 5.0);
        let a2 = Tup::vector(-2.0, 3.0, 1.0);
        assert!((a1 + a2).is_vector());
    }

    #[test]
    fn adding_point_and_vector_produces_a_point() {
        let a1 = Tup::vector(3.0, -2.0, 5.0);
        let a2 = Tup::point(-2.0, 3.0, 1.0);
        assert!((a1 + a2).is_point());
    }

    #[test]
    fn tups_can_be_subtracted() {
        let a1 = Tup::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tup::new(-2.0, 3.0, 1.0, 0.0);
        let expected = Tup::new(5.0, -5.0, 4.0, 1.0);
        assert_eq!(expected, a1 - a2);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_yields_a_point() {
        let p1 = Tup::point(3.0, 2.0, 1.0);
        let v1 = Tup::vector(5.0, 6.0, 7.0);
        let expected = Tup::point(-2.0, -4.0, -6.0);
        assert_eq!(expected, p1 - v1);
    }

    #[test]
    fn subtracting_two_vectors_yields_a_vector() {
        let v1 = Tup::vector(3.0, 2.0, 1.0);
        let v2 = Tup::vector(5.0, 6.0, 7.0);
        let expected = Tup::vector(-2.0, -4.0, -6.0);
        assert_eq!(expected, v1 - v2);
    }

    #[test]
    fn subtacting_a_vector_from_zero_vector_negates_it() {
        let zero = Tup::vector(0.0, 0.0, 0.0);
        let v = Tup::vector(1.0, -2.0, 3.0);
        let expected = Tup::vector(-1.0, 2.0, -3.0);
        assert_eq!(expected, zero - v);
    }

    #[test]
    fn tups_can_be_negated() {
        let t = Tup::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tup::new(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(expected, -t);
    }

    #[test]
    fn a_tup_can_be_multiplied_by_a_scaler() {
        let t = Tup::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tup::new(3.5, -7.0, 10.5, -14.0);
        assert_eq!(expected, t * 3.5);
    }

    #[test]
    fn a_tup_can_be_multiplied_by_a_fractional_scalar() {
        let t = Tup::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tup::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(expected, t * 0.5);
    }

    #[test]
    fn a_tup_can_be_divided_by_a_scalar() {
        let t = Tup::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tup::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(expected, t / 2.0);
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Tup::vector(1.0, 0.0, 0.0);
        assert_nearly_eq(1.0, v.magnitude());
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Tup::vector(0.0, 1.0, 0.0);
        assert_nearly_eq(1.0, v.magnitude());
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Tup::vector(0.0, 0.0, 1.0);
        assert_nearly_eq(1.0, v.magnitude());
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Tup::vector(1.0, 2.0, 3.0);
        assert_nearly_eq(14.0_f64.sqrt(), v.magnitude());
    }

    #[test]
    fn magnitude_of_vector_neg_1_neg_2_neg_3() {
        let v = Tup::vector(-1.0, -2.0, -3.0);
        assert_nearly_eq(14.0_f64.sqrt(), v.magnitude());
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = Tup::vector(4.0, 0.0, 0.0);
        let expected = Tup::vector(1.0, 0.0, 0.0);
        assert_eq!(expected, v.normalize());
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let v = Tup::vector(1.0, 2.0, 3.0);
        let expected = Tup::vector(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt()
        );
        assert_eq!(expected, v.normalize());
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector_is_1() {
        let v = Tup::vector(1.6, -2.4, 3.3);
        assert_nearly_eq(1.0, v.normalize().magnitude());
    }

    #[test]
    fn two_tups_have_a_dot_product() {
        let a = Tup::vector(1.0, 2.0, 3.0);
        let b = Tup::vector(2.0, 3.0, 4.0);
        let expected = 20.0;
        assert_nearly_eq(expected, a.dot(b));
    }

    #[test]
    fn two_vectors_have_a_cross_product() {
        let a = Tup::vector(1.0, 2.0, 3.0);
        let b = Tup::vector(2.0, 3.0, 4.0);
        assert_eq!(Tup::vector(-1.0, 2.0, -1.0), a.cross(b));
        assert_eq!(Tup::vector(1.0, -2.0, 1.0), b.cross(a));
    }

}
