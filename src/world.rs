use crate::color::consts;
use crate::color::Color;
use crate::intersections::{Computations, Intersections};
use crate::lights::Light;
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tup::Tup;
use core::ops::{Index, IndexMut};

type Object = Box<dyn Shape>;

pub struct World {
    light: Light,
    objects: Vec<Object>,
}

impl World {
    pub fn with_light(self, light: Light) -> Self {
        Self { light, ..self }
    }

    pub fn with_object<T: 'static + Shape>(mut self, shape: T) -> Self {
        self.objects.push(Box::new(shape));
        self
    }

    pub fn light(&self) -> Light {
        self.light
    }

    pub fn num_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut intersections = Intersections::default();
        for object in self.objects.iter() {
            let inters = object.intersect(&ray);
            intersections = intersections.append(inters);
        }
        intersections
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        let shadowed = self.is_shadowed(comps.over_point());
        comps.object().material().lighting(
            self.light,
            comps.over_point(),
            comps.eyev(),
            comps.normalv(),
            shadowed,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        self.intersect(ray)
            .hit()
            .map(|i| self.shade_hit(i.clone().prepare_computations(ray)))
            .unwrap_or(consts::BLACK)
    }

    pub fn is_shadowed(&self, point: Tup) -> bool {
        let point_to_lightv = self.light().position() - point;
        let distance = point_to_lightv.magnitude();
        let ray = Ray::new(point, point_to_lightv.normalize());
        let inters = self.intersect(ray);
        inters.hit().map_or(false, |i| i.t() < distance)
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            light: Light::point_light(Tup::point(0, 0, 0), consts::BLACK),
            objects: Vec::new(),
        }
    }
}

impl Index<usize> for World {
    type Output = Object;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl IndexMut<usize> for World {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}

#[cfg(test)]
mod world_test {
    use super::*;
    use crate::intersections::Intersection;
    use crate::materials::Material;
    use crate::spheres::Sphere;
    use crate::test_helpers::{assert_nearly_eq, default_test_world};
    use crate::transforms::translation;

    #[test]
    fn an_new_world_has_default_black_light_source() {
        let world = World::default();
        assert_eq!(
            world.light(),
            Light::point_light(Tup::point(0, 0, 0), Color::new(0, 0, 0))
        );
    }

    #[test]
    fn an_new_world_has_no_objects() {
        let world = World::default();
        assert_eq!(0, world.num_objects())
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = default_test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let xs = world.intersect(r);
        assert_eq!(4, xs.len());
        assert_nearly_eq(4.0, xs[0].t());
        assert_nearly_eq(4.5, xs[1].t());
        assert_nearly_eq(5.5, xs[2].t());
        assert_nearly_eq(6.0, xs[3].t());
    }

    #[test]
    fn shading_an_intersection_from_the_outside() {
        let w = default_test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = w[0].clone();
        let i = Intersection::from_boxed_shape(4, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = default_test_world().with_light(Light::point_light(
            Tup::point(0.0, 0.25, 0.0),
            Color::new(1, 1, 1),
        ));
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let shape = w[1].clone();
        let i = Intersection::from_boxed_shape(0.5, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn the_color_when_a_ray_misses_an_object() {
        let w = default_test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 1, 0));
        let c = w.color_at(r);
        assert_eq!(Color::new(0, 0, 0), c);
    }

    #[test]
    fn the_color_when_a_ray_hits_an_object_from_outside() {
        let w = default_test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let c = w.color_at(r);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn the_color_with_intersection_behind_a_ray() {
        let mut w = default_test_world();
        let material = Material::default().with_ambient(1.0);
        let outer = &mut w[0];
        outer.set_material(material);
        let inner = &mut w[1];
        inner.set_material(material);
        let inner_color = inner.material().color();
        let r = Ray::new(Tup::point(0.0, 0.0, 0.75), Tup::vector(0, 0, -1));
        let c = w.color_at(r);
        assert_eq!(inner_color, c);
    }

    #[test]
    fn no_shadows_when_nothing_is_colinear_with_point_and_light() {
        let world = default_test_world();
        let p = Tup::point(0, 10, 0);
        assert!(!world.is_shadowed(p));
    }

    #[test]
    fn is_shadowed_when_object_between_point_and_light() {
        let world = default_test_world();
        let p = Tup::point(10, -10, 10);
        assert!(world.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let world = default_test_world();
        let p = Tup::point(-20, 20, -20);
        assert!(!world.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let world = default_test_world();
        let p = Tup::point(-2, 2, -2);
        assert!(!world.is_shadowed(p));
    }

    #[test]
    fn shade_hit_responds_correctly_when_given_an_intersection_in_shadow() {
        let light = Light::point_light(Tup::point(0, 0, -10), consts::WHITE);
        let s1 = Sphere::default();
        let s2 = Sphere::default().with_transform(translation(0, 0, 10));
        let world = World::default()
            .with_light(light)
            .with_object(s1)
            .with_object(s2);
        let ray = Ray::new(Tup::point(0, 0, 5), Tup::vector(0, 0, 1));
        let i = Intersection::new(4, s2);
        let comps = i.prepare_computations(ray);
        let color = world.shade_hit(comps);
        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
    }
}
