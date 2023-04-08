use crate::rayst::color::consts as col;
use crate::rayst::color::Color;
use crate::rayst::intersections::{Computations, Intersections};
use crate::rayst::lights::Light;
use crate::rayst::math_helpers::nearly_eq;
use crate::rayst::rays::Ray;
use crate::rayst::shapes::Shape;
use crate::rayst::tup::Tup;
use std::ops::{Index, IndexMut};

type Object = Box<dyn Shape>;

#[derive(Debug)]
pub struct World {
    light: Light,
    objects: Vec<Object>,
}

impl World {
    pub const MAX_BOUNCES: usize = 5;

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

    pub fn shade_hit(&self, comps: &Computations, remaining_bounces: usize) -> Color {
        let shadowed = self.is_shadowed(comps.over_point());
        let surface = comps.object().material().lighting(
            comps.object().transform(),
            self.light,
            comps.over_point(),
            comps.eyev(),
            comps.normalv(),
            shadowed,
        );
        let reflection = self.reflected_color(comps, remaining_bounces);
        let refraction = self.refracted_color(comps, remaining_bounces);
        surface + reflection + refraction
    }

    fn calc_reflected(&self, comps: &Computations, remaining_bounces: usize) -> Color {
        let reflective = comps.object().material().reflective();
        if nearly_eq(0.0, reflective) {
            col::BLACK
        } else {
            let r = Ray::new(comps.over_point(), comps.reflectv());
            self.color_at(r, remaining_bounces - 1) * reflective
        }
    }

    pub fn reflected_color(&self, comps: &Computations, remaining_bounces: usize) -> Color {
        if remaining_bounces == 0 {
            col::BLACK
        } else {
            self.calc_reflected(comps, remaining_bounces)
        }
    }

    pub fn color_at(&self, ray: Ray, remaining_bounces: usize) -> Color {
        let intersections = self.intersect(ray);
        intersections
            .hit()
            .map(|i| {
                self.shade_hit(
                    &i.prepare_computations(&ray, &intersections),
                    remaining_bounces,
                )
            })
            .unwrap_or(col::BLACK)
    }

    pub fn is_shadowed(&self, point: Tup) -> bool {
        let point_to_lightv = self.light().position() - point;
        let distance = point_to_lightv.magnitude();
        let ray = Ray::new(point, point_to_lightv.normalize());
        let inters = self.intersect(ray);
        inters.hit().map_or(false, |i| i.t() < distance)
    }

    pub fn refracted_color(&self, comps: &Computations, remaining_bounces: usize) -> Color {
        let transparency = comps.object().material().transparency();
        let n_ratio = comps.n1() / comps.n2();
        let cos_i = comps.eyev().dot(&comps.normalv());
        let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos_i * cos_i));
        let total_internal_reflection = sin2_t > 1.0;
        if remaining_bounces < 1 || total_internal_reflection || nearly_eq(0.0, transparency) {
            return col::BLACK;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv() * (n_ratio * cos_i - cos_t) - comps.eyev() * n_ratio;
        let refract_ray = Ray::new(comps.under_point(), direction);
        self.color_at(refract_ray, remaining_bounces - 1) * comps.object().material().transparency()
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            light: Light::point_light(Tup::point(0, 0, 0), col::BLACK),
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
    use crate::rayst::intersections::Intersection;
    use crate::rayst::materials::Material;
    use crate::rayst::patterns::Pattern;
    use crate::rayst::planes::Plane;
    use crate::rayst::spheres::Sphere;
    use crate::rayst::test_helpers::{assert_nearly_eq, default_test_world};
    use crate::rayst::transforms::{self, translation};
    use std::f64::consts;

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
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        let c = w.shade_hit(&comps, World::MAX_BOUNCES);
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
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        let c = w.shade_hit(&comps, World::MAX_BOUNCES);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn the_color_when_a_ray_misses_an_object() {
        let w = default_test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 1, 0));
        let c = w.color_at(r, World::MAX_BOUNCES);
        assert_eq!(Color::new(0, 0, 0), c);
    }

    #[test]
    fn the_color_when_a_ray_hits_an_object_from_outside() {
        let w = default_test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let c = w.color_at(r, World::MAX_BOUNCES);
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
        let c = w.color_at(r, World::MAX_BOUNCES);
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
        let light = Light::point_light(Tup::point(0, 0, -10), col::WHITE);
        let s1 = Sphere::default();
        let s2 = Sphere::default().with_transform(translation(0, 0, 10));
        let world = World::default()
            .with_light(light)
            .with_object(s1)
            .with_object(s2);
        let ray = Ray::new(Tup::point(0, 0, 5), Tup::vector(0, 0, 1));
        let i = Intersection::new(4, s2);
        let comps = i.prepare_computations(&ray, &Intersections::new(&[i.clone()]));
        let color = world.shade_hit(&comps, World::MAX_BOUNCES);
        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material_is_black() {
        let world = default_test_world();
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let mut shape = world[1].clone();
        let current_material = shape.material();
        shape.set_material(current_material.with_ambient(1.0));
        let i = Intersection::from_boxed_shape(1.0, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        let color = world.reflected_color(&comps, World::MAX_BOUNCES);
        assert_eq!(col::BLACK, color);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let material = Material::default().with_reflective(0.5);
        let shape = Plane::default()
            .with_material(material)
            .with_transform(translation(0, -1, 0));
        let world = default_test_world().with_object(shape);
        let rad_2 = 2.0_f64.sqrt();
        let rad_2_over_2 = rad_2 / 2.0;
        let r = Ray::new(
            Tup::point(0, 0, -3),
            Tup::vector(0.0, -rad_2_over_2, rad_2_over_2),
        );
        let i = Intersection::new(rad_2, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        let color = world.reflected_color(&comps, World::MAX_BOUNCES);
        assert_eq!(Color::new(0.19033, 0.23791, 0.14274), color);
    }

    #[test]
    fn shade_hit_incorporates_the_reflected_color() {
        let material = Material::default().with_reflective(0.5);
        let shape = Plane::default()
            .with_material(material)
            .with_transform(translation(0, -1, 0));
        let world = default_test_world().with_object(shape);
        let rad_2 = 2.0_f64.sqrt();
        let rad_2_over_2 = rad_2 / 2.0;
        let r = Ray::new(
            Tup::point(0, 0, -3),
            Tup::vector(0.0, -rad_2_over_2, rad_2_over_2),
        );
        let i = Intersection::new(rad_2, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        let color = world.shade_hit(&comps, World::MAX_BOUNCES);
        assert_eq!(Color::new(0.87676, 0.92435, 0.82918), color);
    }

    #[test]
    fn mutually_reflective_surfaces_dont_cause_infinite_recursion() {
        let lower = Plane::default()
            .with_material(Material::default().with_reflective(1.0))
            .with_transform(translation(0, -1, 0));
        let upper = Plane::default()
            .with_material(Material::default().with_reflective(1.0))
            .with_transform(translation(0, 1, 0));
        let world = World::default()
            .with_light(Light::point_light(Tup::point(0, 0, 0), col::WHITE))
            .with_object(lower)
            .with_object(upper);
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 1, 0));
        let result = world.color_at(r, World::MAX_BOUNCES);
        assert!(result.red() > 1.0 && result.green() > 1.0 && result.blue() > 1.0);
    }

    #[test]
    fn the_reflected_color_at_max_recursion_depth_is_black() {
        let shape = Plane::default()
            .with_material(Material::default().with_reflective(0.5))
            .with_transform(translation(0, -1, 0));
        let world = default_test_world().with_object(shape);
        // let rad_2 = 2.0_f64.sqrt();
        let rad_2 = std::f64::consts::SQRT_2;
        let rad_2_over_2 = rad_2 / 2.0;
        let r = Ray::new(
            Tup::point(0, 0, -3),
            Tup::vector(0.0, -rad_2_over_2, rad_2_over_2),
        );
        let i = Intersection::new(rad_2, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        let color = world.reflected_color(&comps, 0);
        assert_eq!(col::BLACK, color);
    }

    #[test]
    fn the_refracted_color_of_an_opaque_object_is_black() {
        let w = default_test_world();
        let shape = w[0].clone();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let xs = Intersections::new(&[
            Intersection::from_boxed_shape(4.0, shape.clone()),
            Intersection::from_boxed_shape(6.0, shape.clone()),
        ]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let color = w.refracted_color(&comps, World::MAX_BOUNCES);
        assert_eq!(col::BLACK, color);
    }

    #[test]
    fn the_refracted_color_at_max_recursive_depth_is_black() {
        let w = default_test_world();
        let shape = &w[0];
        let material = shape.material();
        material.with_transparency(1.0).with_refractive_index(1.5);
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let xs = Intersections::new(&[
            Intersection::from_boxed_shape(4.0, shape.clone()),
            Intersection::from_boxed_shape(6.0, shape.clone()),
        ]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let color = w.refracted_color(&comps, 0);
        assert_eq!(col::BLACK, color);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection_is_black() {
        let w = default_test_world();
        let shape = &w[0];
        let material = shape.material();
        material.with_transparency(1.0).with_refractive_index(1.5);
        let rad_2_over_2 = consts::SQRT_2 / 2.0;
        let r = Ray::new(Tup::point(0.0, 0.0, rad_2_over_2), Tup::vector(0, 1, 0));
        let xs = Intersections::new(&[
            Intersection::from_boxed_shape(-rad_2_over_2, shape.clone()),
            Intersection::from_boxed_shape(rad_2_over_2, shape.clone()),
        ]);
        let comps = xs[1].prepare_computations(&r, &xs);
        let color = w.refracted_color(&comps, 5);
        assert_eq!(col::BLACK, color);
    }

    fn refracted_color_test_world() -> World {
        let light = Light::point_light(Tup::point(-10, 10, -10), Color::new(1, 1, 1));

        let material_1 = Material::default()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2)
            .with_ambient(1.0)
            .with_pattern(Pattern::default());
        let s1 = Sphere::default().with_material(material_1);

        let transform = transforms::scaling(0.5, 0.5, 0.5);
        let material_2 = Material::default()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        let s2 = Sphere::default()
            .with_transform(transform)
            .with_material(material_2);

        World::default()
            .with_light(light)
            .with_object(s1)
            .with_object(s2)
    }

    #[test]
    fn the_refracted_color_is_determined_from_a_refracted_ray() {
        let w = refracted_color_test_world();
        let r = Ray::new(Tup::point(0.0, 0.0, 0.1), Tup::vector(0, 1, 0));
        let xs = Intersections::new(&[
            Intersection::from_boxed_shape(-0.9899, w[0].clone()),
            Intersection::from_boxed_shape(-0.4899, w[1].clone()),
            Intersection::from_boxed_shape(0.4899, w[1].clone()),
            Intersection::from_boxed_shape(0.9899, w[0].clone()),
        ]);
        let comps = xs[2].prepare_computations(&r, &xs);
        let color = w.refracted_color(&comps, 5);
        assert_eq!(Color::new(0.0, 0.99888, 0.04722), color);
    }

    fn shade_hit_refraction_test_world() -> World {
        let floor_material = Material::default()
            .with_transparency(0.5)
            .with_refractive_index(1.5);
        let floor = Plane::default()
            .with_transform(transforms::translation(0, -1, 0))
            .with_material(floor_material);

        let ball_material = Material::default().with_color(col::RED).with_ambient(0.5);
        let ball = Sphere::default()
            .with_transform(transforms::translation(0.0, -3.5, -0.5))
            .with_material(ball_material);

        default_test_world().with_object(floor).with_object(ball)
    }

    #[test]
    fn the_shaade_hit_color_is_determined_from_a_refracted_ray() {
        let w = shade_hit_refraction_test_world();
        dbg!(&w);
        let rad_2_over_2 = consts::SQRT_2 / 2.0;
        let r = Ray::new(
            Tup::point(0.0, 0.0, -3.0),
            Tup::vector(0.0, -rad_2_over_2, rad_2_over_2),
        );
        let xs =
            Intersections::new(&[Intersection::from_boxed_shape(consts::SQRT_2, w[2].clone())]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let color = w.shade_hit(&comps, 5);
        let expected = Color::new(0.93642, 0.68642, 0.68642);
        assert_eq!(expected, color);
    }
}
