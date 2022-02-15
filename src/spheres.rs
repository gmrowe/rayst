use crate::tup::Tup;
use crate::rays::Ray;

struct Sphere {
    
}

impl Sphere {
    fn new() -> Self {
        Self {
   
        }
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let center_of_sphere = Tup::point(0.0, 0.0, 0.0);
        let sphere_to_ray_vec = ray.origin() - center_of_sphere;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray_vec);
        let c = sphere_to_ray_vec.dot(&sphere_to_ray_vec) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }
}

#[cfg(test)]
mod spheres_test {
    use super::*;
    use crate::math_helpers;

    fn assert_nearly_eq(a: f64, b: f64) {
        assert!(math_helpers::nearly_eq(a, b));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn can_determine_the_first_ray_sphere_intersection() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(4.0, xs[0]);
    }

    #[test]
    fn can_determine_the_second_ray_sphere_intersection() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(6.0, xs[1]);
    }
    
    #[test]
    fn a_ray_intersecting_a_sphere_at_a_tangent_reutrns_two_intersections() {
        let ray = Ray::new(Tup::point(0.0, 1.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());        
    }

    #[test]
    fn both_tangent_intersections_returned_are_same() {
        let ray = Ray::new(Tup::point(0.0, 1.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(xs[0], xs[1]);
        assert_nearly_eq(5.0, xs[0]);
    }

    #[test]
    fn a_ray_can_completely_miss_a_sphere() {
        let ray = Ray::new(Tup::point(0.0, 2.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originating_from_inside_a_sphere_has_two_intersections() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 0.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());        
    }

    #[test]
    fn a_ray_originating_from_inside_a_sphere_intersects_in_negative_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 0.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(-1.0, xs[0]);        
    }
    
    #[test]
    fn a_ray_originating_from_inside_a_sphere_intersects_in_positive_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 0.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(1.0, xs[1]);        
    }

    #[test]
    fn a_ray_in_front_of_sphere_has_two_intersections() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn a_ray_in_front_of_sphere_has_intersects_in_negative_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(-6.0, xs[0]);
    }

    #[test]
    fn a_ray_in_front_of_sphere_has_intersects_a_second_time_in_negative_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(-4.0, xs[1]);
    }
}
