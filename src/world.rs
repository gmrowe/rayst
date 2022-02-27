use crate::color::Color;
use crate::lights::Light;
use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::tup::Tup;
use crate::intersections::{Intersections, Computations};

#[derive(PartialEq, Debug, Clone)]
pub struct World {
    light: Light,
    objects: Vec<Sphere>,
}

impl World {
    pub fn with_light(self, light: Light) -> Self {
        Self {
            light,
            ..self
        }
    }

    pub fn with_object(mut self, sphere: Sphere) -> Self {
        self.objects.push(sphere);
        self
    }
    
    pub fn light(&self) -> Light {
        self.light
    }

    pub fn objects(&self) -> &[Sphere] {
        &self.objects
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
        comps.object()
            .material()
            .lighting(self.light, comps.point(), comps.eyev(), comps.normalv())
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        self.intersect(ray).hit()
            .map(|i| self.shade_hit(i.prepare_computations(ray)))
            .unwrap_or(Color::new(0, 0, 0))
    }
}


impl Default for World {
    fn default() -> Self {
        Self {
            light: Light::point_light(Tup::point(0, 0, 0), Color::new(0, 0, 0)),
            objects: Vec::new(),
        }
    }
}

#[cfg(test)]
mod world_test {
    use super::*;
    use crate::materials::Material;
    use crate::intersections::Intersection;
    use crate::transforms;
    use crate::math_helpers::nearly_eq;

    fn default_test_world() -> World {
        let light =
            Light::point_light(Tup::point(-10, 10, -10), Color::new(1, 1, 1));
        
        let material = Material::default()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Sphere::default().with_material(material);
        
        let transform = transforms::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::default().with_transform(transform);
        
        World::default()
            .with_light(light)
            .with_object(s1)
            .with_object(s2)
    }

    fn assert_nearly_eq(a: f64, b: f64) {
        assert!(nearly_eq(a, b));
    }

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
        assert!(world.objects().is_empty());
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
        let shape = w.objects()[0];
        let i = Intersection::new(4, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = default_test_world()
            .with_light(Light::point_light(Tup::point(0.0, 0.25, 0.0), Color::new(1, 1, 1)));
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let shape = w.objects()[1];
        let i = Intersection::new(0.5, shape);
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
        w.objects[0] = w.objects[0].with_material(material);
        w.objects[1] = w.objects[0].with_material(material);
        let inner = w.objects()[1];
        let r = Ray::new(Tup::point(0.0, 0.0, 0.75), Tup::vector(0, 0, -1));
        let c = w.color_at(r);
        assert_eq!(inner.material().color(), c);
    }    
}
