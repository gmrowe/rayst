use crate::rayst::matrix::Mat4;
use crate::rayst::tup::Tup;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    origin: Tup,    // point
    direction: Tup, // vector
}

impl Ray {
    pub fn new(origin: Tup, direction: Tup) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Tup {
        self.origin
    }

    pub fn direction(&self) -> Tup {
        self.direction
    }

    pub fn position(&self, distance: f64) -> Tup {
        self.direction() * distance + self.origin()
    }

    pub fn transform(&self, mat: &Mat4) -> Self {
        Self::new(*mat * self.origin(), *mat * self.direction())
    }
}

#[cfg(test)]
mod rays_test {
    use super::*;
    use crate::rayst::transforms;

    #[test]
    fn a_ray_has_an_origin() {
        let origin = Tup::point(1.0, 2.0, 3.0);
        let direction = Tup::vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(origin, ray.origin());
    }

    #[test]
    fn a_ray_has_an_direction() {
        let origin = Tup::point(1.0, 2.0, 3.0);
        let direction = Tup::vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(direction, ray.direction());
    }

    #[test]
    fn a_point_can_be_computed_from_a_ray_and_distance() {
        let ray = Ray::new(Tup::point(2.0, 3.0, 4.0), Tup::vector(1.0, 0.0, 0.0));
        assert_eq!(Tup::point(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Tup::point(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Tup::point(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Tup::point(4.5, 3.0, 4.0), ray.position(2.5));
    }

    #[test]
    fn when_a_ray_is_translated_its_origin_changes() {
        let ray = Ray::new(Tup::point(1, 2, 3), Tup::vector(0, 1, 0));
        let m = transforms::translation(3, 4, 5);
        let r2 = ray.transform(&m);
        assert_eq!(Tup::point(4, 6, 8), r2.origin());
    }

    #[test]
    fn when_a_ray_is_translated_its_vector_is_unchanged() {
        let ray = Ray::new(Tup::point(1, 2, 3), Tup::vector(0, 1, 0));
        let m = transforms::translation(3, 4, 5);
        let r2 = ray.transform(&m);
        assert_eq!(Tup::vector(0, 1, 0), r2.direction());
    }

    #[test]
    fn when_a_ray_is_scaled_its_origin_changes() {
        let ray = Ray::new(Tup::point(1, 2, 3), Tup::vector(0, 1, 0));
        let m = transforms::scaling(2, 3, 4);
        let r2 = ray.transform(&m);
        assert_eq!(Tup::point(2, 6, 12), r2.origin());
    }

    #[test]
    fn when_a_ray_is_scaled_its_direction_changes() {
        let ray = Ray::new(Tup::point(1, 2, 3), Tup::vector(0, 1, 0));
        let m = transforms::scaling(2, 3, 4);
        let r2 = ray.transform(&m);
        assert_eq!(Tup::vector(0, 3, 0), r2.direction());
    }
}
