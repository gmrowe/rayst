use crate::intersections::{Intersection, Intersections};
use crate::materials::Material;
use crate::math_helpers::EPSILON;
use crate::matrix::Mat4;
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tup::Tup;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Plane {
    transform: Mat4,
    material: Material,
}

impl Plane {
    pub fn with_material(self, material: Material) -> Self {
        Self { material, ..self }
    }

    pub fn with_transform(self, transform: Mat4) -> Self {
        Self { transform, ..self }
    }
}

impl Shape for Plane {
    fn transform(&self) -> Mat4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
    }

    fn material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        if local_ray.direction().y.abs() < EPSILON {
            Intersections::default()
        } else {
            let t = -local_ray.origin().y / local_ray.direction().y;
            Intersections::new(&vec![Intersection::new(t, *self)])
        }
    }

    fn normal_at(&self, point: Tup) -> Tup {
        let inverse_xform = self.transform().inverse();
        let local_point = inverse_xform * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = inverse_xform.transpose() * local_normal;
        // Hack to ensure that w = 1.0 - See pg. 82
        let world_normal_vec = Tup::vector(world_normal.x, world_normal.y, world_normal.z);
        world_normal_vec.normalize()
    }

    fn local_normal_at(&self, _point: Tup) -> Tup {
        Tup::vector(0, 1, 0)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Mat4::default(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod planes_test {
    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();
        let n1 = p.local_normal_at(Tup::point(0, 0, 0));
        let n2 = p.local_normal_at(Tup::point(10, 0, -10));
        let n3 = p.local_normal_at(Tup::point(-5, 0, 150));
        assert_eq!(Tup::vector(0, 1, 0), n1);
        assert_eq!(Tup::vector(0, 1, 0), n2);
        assert_eq!(Tup::vector(0, 1, 0), n3);
    }

    #[test]
    fn a_ray_parallel_to_a_plane_never_intersects_the_plane() {
        let p = Plane::default();
        let r = Ray::new(Tup::point(0, 10, 0), Tup::vector(0, 0, 1));
        let xs = p.local_intersect(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_coplanar_with_a_plane_never_intersects_the_plane() {
        let p = Plane::default();
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let xs = p.local_intersect(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_can_intersect_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray::new(Tup::point(0, 1, 0), Tup::vector(0, -1, 0));
        let xs = p.local_intersect(r);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t());
    }

    #[test]
    fn a_ray_can_intersect_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray::new(Tup::point(0, -1, 0), Tup::vector(0, 1, 0));
        let xs = p.local_intersect(r);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t());
    }
}
