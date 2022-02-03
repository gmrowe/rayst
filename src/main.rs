fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Tup {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    
}

impl Tup {
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
}

fn nearly_eq(a: f64, b: f64) -> bool {
    let diff = (a - b).abs();
    diff.abs() < f64::EPSILON
}

#[cfg(test)]
mod tup_tests {
    use super::*;

    #[test]
    fn a_tup_stores_its_x_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert!(nearly_eq(tup.x, 4.3));
    }

    #[test]
    fn a_tup_stores_ites_y_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert!(nearly_eq(tup.y, -4.2));
    }

    #[test]
    fn a_tup_stores_ites_z_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert!(nearly_eq(tup.z, 3.1));
    }

    #[test]
    fn a_tup_stores_ites_w_value() {
        let tup = Tup::new(4.3, -4.2, 3.1, 1.0);
        assert!(nearly_eq(tup.w, 1.0));
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
}
