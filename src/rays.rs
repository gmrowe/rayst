use crate::Tup;

struct Ray {
    origin: Tup, // point
    direction: Tup, // vector
}

impl Ray {
    fn new(origin: Tup, direction: Tup) -> Self {
        Self {
            origin,
            direction,
        }
    }

    fn origin(&self) -> Tup {
        self.origin
    }

    fn direction(&self) -> Tup {
        self.direction
    }

    fn position(&self, distance: f64) -> Tup {
        self.direction() * distance + self.origin()
    }
}

#[cfg(test)]
mod rays_test {
    use super::*;

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
}
