use rayst::camera::Camera;
use rayst::color::consts as col;
use rayst::color::Color;
use rayst::lights::Light;
use rayst::materials::Material;
use rayst::patterns::Pattern;
use rayst::planes::Plane;
use rayst::spheres::Sphere;
use rayst::transforms;
use rayst::tup::Tup;
use rayst::world::World;
use std::f64::consts;

const SCREEN_FACTOR: usize = 5;
const CANVAS_WIDTH: usize = 300 * SCREEN_FACTOR;
const CANVAS_HEIGHT: usize = 180 * SCREEN_FACTOR;
const CAMERA_FIELD_OF_VIEW: f64 = consts::FRAC_PI_2 * 1.2;

fn camera() -> Camera {
    let from = Tup::point(0.0, 3.0, -5.0);
    let to = Tup::point(0.0, 0.0, -1.0);
    let up = Tup::vector(0.0, 1.0, 0.0);
    let camera_transform = transforms::view_transform(from, to, up);
    Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, CAMERA_FIELD_OF_VIEW)
        .with_transform(camera_transform)
        .with_progress_logging()
}

fn light_source() -> Light {
    let light_position = Tup::point(-10, 10, -10);
    let light_intensity = Color::new(1, 1, 1);
    Light::point_light(light_position, light_intensity)
}

fn floor() -> Plane {
    let eggshell_white = Color::from_hex(0xF0EAD6);
    let material =
        Material::default().with_pattern(Pattern::checkers_pattern(col::OLIVE, eggshell_white));
    Plane::default().with_material(material)
}

fn mirror_sphere(x: f64, y: f64, z: f64) -> Sphere {
    let radius = 1.0;
    let ambient = 0.1;
    let diffuse = 0.01;
    let specular = 0.8;
    let reflective = 1.0;
    let material = Material::default()
        .with_color(Color::from_hex(0x101010))
        .with_diffuse(diffuse)
        .with_reflective(reflective)
        .with_ambient(ambient)
        .with_refractive_index(1.9)
        .with_specular(specular);

    Sphere::default()
        .with_transform(
            transforms::scaling(radius, radius, radius) * transforms::translation(x, y, z),
        )
        .with_material(material)
}

fn sphere_in_a_sphere(x: f64, y: f64, z: f64, color: Color) -> [Sphere; 2] {
    let outer_radius = 1.0;
    let outer_ambient = 0.1;
    let outer_diffuse = 0.1;
    let outer_specular = 0.3;
    let outer_reflective = 0.5;
    let outer_transparancy = 1.0;
    let outer_refractive_index = 1.5;
    let outer_material = Material::default()
        .with_diffuse(outer_diffuse)
        .with_reflective(outer_reflective)
        .with_ambient(outer_ambient)
        .with_refractive_index(outer_refractive_index)
        .with_transparency(outer_transparancy)
        .with_specular(outer_specular);
    let outer_transform = transforms::translation(x, y, z)
        * transforms::scaling(outer_radius, outer_radius, outer_radius);
    let outer = Sphere::default()
        .with_transform(outer_transform)
        .with_material(outer_material);

    let inner_radius = 0.33;
    let inner_ambient = 0.3;
    let inner_diffuse = 0.7;
    let inner_specular = 0.3;
    let inner_reflective = 0.1;
    let inner_transparancy = 0.0;
    let inner_material = Material::default()
        .with_color(color)
        .with_diffuse(inner_diffuse)
        .with_reflective(inner_reflective)
        .with_ambient(inner_ambient)
        .with_transparency(inner_transparancy)
        .with_specular(inner_specular);
    let inner_transform = transforms::translation(x, y, z)
        * transforms::scaling(inner_radius, inner_radius, inner_radius);
    let inner = Sphere::default()
        .with_transform(inner_transform)
        .with_material(inner_material);

    [outer, inner]
}

fn solid_sphere(x: f64, y: f64, z: f64, color: Color) -> Sphere {
    let radius = 1.0;
    let ambient = 0.3;
    let diffuse = 0.7;
    let specular = 0.8;
    let reflective = 0.1;
    let transparancy = 0.0;
    let material = Material::default()
        .with_color(color)
        .with_diffuse(diffuse)
        .with_reflective(reflective)
        .with_ambient(ambient)
        .with_transparency(transparancy)
        .with_specular(specular);
    let transform = transforms::translation(x, y, z) * transforms::scaling(radius, radius, radius);
    Sphere::default()
        .with_transform(transform)
        .with_material(material)
}

fn back_wall() -> Plane {
    let shape = Plane::default();
    let transform =
        transforms::translation(0.0, 0.0, 2.5) * transforms::rotation_x(consts::FRAC_PI_2);
    shape.with_transform(transform)
}

fn scene() -> World {
    let spheres = sphere_in_a_sphere(0.0, 1.0, 1.0, col::RED);
    World::default()
        .with_light(light_source())
        .with_object(floor())
        .with_object(spheres[0])
        .with_object(spheres[1])
        .with_object(solid_sphere(1.5, 1.0, -2.5, col::MAGENTA))
        .with_object(back_wall())
        .with_object(mirror_sphere(-2.0, 1.0, -1.8))
}

fn main() -> std::io::Result<()> {
    let canvas = camera().render(&scene());
    let png = canvas.to_png();
    let image_name = "reflect_and_refract.png";
    std::fs::write(image_name, png)?;
    Ok(())
}
