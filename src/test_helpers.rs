use crate::color::Color;
use crate::lights::Light;
use crate::materials::Material;
use crate::math_helpers::nearly_eq;
use crate::spheres::Sphere;
use crate::transforms;
use crate::tup::Tup;
use crate::world::World;

pub fn assert_nearly_eq(a: f64, b: f64) {
    assert!(nearly_eq(a, b));
}

pub fn default_test_world() -> World {
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


