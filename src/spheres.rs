use crate::matrix::Mat4;
use crate::tup::Tup;
use crate::rays::Ray;
use crate::intersections::Intersection;
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_GEN: AtomicUsize = AtomicUsize::new(0);

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Sphere {
    id: usize,
    transform: Mat4,
}

fn get_id() -> usize {
    ID_GEN.fetch_add(1, Ordering::Relaxed)
}

impl Sphere {
    pub fn intersect(&self, ray_object_space: &Ray) -> Vec<Intersection> {
        let center_of_sphere = Tup::point(0.0, 0.0, 0.0);
        let ray_world_space = ray_object_space.transform(&self.transform().inverse());
        let sphere_to_ray_vec = ray_world_space.origin() - center_of_sphere;
        let a = ray_world_space.direction().dot(&ray_world_space.direction());
        let b = 2.0 * ray_world_space.direction().dot(&sphere_to_ray_vec);
        let c = sphere_to_ray_vec.dot(&sphere_to_ray_vec) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *self);
            let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *self);
            Intersection::intersections(t1, t2)
        }
    }

    fn transform(&self) -> Mat4 {
        self.transform
    }

    fn set_transform(self, transform: Mat4) -> Self {
        Self {
            transform,
            ..self
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: get_id(),
            transform: Mat4::identity_matrix(),
        }
    }
}

#[cfg(test)]
mod spheres_test {
    use super::*;
    use crate::math_helpers;
    use crate::transforms;

    fn assert_nearly_eq(a: f64, b: f64) {
        assert!(math_helpers::nearly_eq(a, b));
    }

    #[test]
    fn two_spheres_are_not_the_same() {
        let s1 = Sphere::default();
        let s2 = Sphere::default();
        assert_ne!(s1, s2);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn can_determine_the_first_ray_sphere_intersection() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(4.0, xs[0].t());
    }

    #[test]
    fn can_determine_the_second_ray_sphere_intersection() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(6.0, xs[1].t());
    }

    #[test]
    fn the_object_of_both_intersections_is_the_same() {
        let ray = Ray::new(Tup::point(0.0, 0.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq!(sphere, xs[0].object());
        assert_eq!(sphere, xs[1].object());
    }    
    
    #[test]
    fn a_ray_intersecting_a_sphere_at_a_tangent_reutrns_two_intersections() {
        let ray = Ray::new(Tup::point(0.0, 1.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());        
    }

    #[test]
    fn both_tangent_intersections_returned_are_same() {
        let ray = Ray::new(Tup::point(0.0, 1.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(xs[0].t(), xs[1].t());
        assert_nearly_eq(5.0, xs[0].t());
    }

    #[test]
    fn a_ray_can_completely_miss_a_sphere() {
        let ray = Ray::new(Tup::point(0.0, 2.0, -5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originating_from_inside_a_sphere_has_two_intersections() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 0.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());        
    }

    #[test]
    fn a_ray_originating_from_inside_a_sphere_intersects_in_negative_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 0.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(-1.0, xs[0].t());        
    }
    
    #[test]
    fn a_ray_originating_from_inside_a_sphere_intersects_in_positive_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 0.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(1.0, xs[1].t());        
    }

    #[test]
    fn a_ray_in_front_of_sphere_has_two_intersections() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn a_ray_in_front_of_sphere_has_intersects_in_negative_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(-6.0, xs[0].t());
    }

    #[test]
    fn a_ray_in_front_of_sphere_has_intersects_a_second_time_in_negative_distance() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_nearly_eq(-4.0, xs[1].t());
    }

    #[test]
    fn intersect_sets_the_object_of_the_intersecton() {
        let ray = Ray::new(Tup::point(0.0, 0.0, 5.0), Tup::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let xs = sphere.intersect(&ray);
        assert_eq! (sphere, xs[0].object()); 
    }

    #[test]
    fn a_spheres_default_transformation_is_the_identity_matrix() {
        let sphere = Sphere::default();
        assert_eq!(Mat4::identity_matrix(), sphere.transform())
    }

    #[test]
    fn a_spheres_transform_can_be_set() {
        let mut sphere = Sphere::default();
        let t = transforms::translation(2, 3, 4);
        sphere = sphere.set_transform(t);
        assert_eq!(t, sphere.transform())
    }

    #[test]
    fn a_sphere_transforms_a_ray_before_calculating_intersects_when_scaled() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let mut s = Sphere::default();
        s = s.set_transform(transforms::scaling(2, 2, 2));
        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());
        assert_nearly_eq(3.0, xs[0].t());
        assert_nearly_eq(7.0, xs[1].t());
    }

    #[test]
    fn a_sphere_transforms_a_ray_before_calculating_intersects_when_translated() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let mut s = Sphere::default();
        s = s.set_transform(transforms::translation(5, 0, 0));
        let xs = s.intersect(&r);
        assert_eq!(0, xs.len());
    }
}
