use rayst::camera::Camera;
use rayst::canvas::Canvas;
use rayst::color::consts as col;
use rayst::color::Color;
use rayst::lights::Light;
use rayst::materials::Material;
use rayst::math_helpers;
use rayst::patterns::Pattern;
use rayst::planes::Plane;
use rayst::spheres::Sphere;
use rayst::transforms;
use rayst::tup::Tup;
use rayst::world::World;
use std::f64::consts;
use std::fs;

const CANVAS_WIDTH: usize = 300 * 2;
const CANVAS_HEIGHT: usize = 180 * 2;
const CAMERA_FIELD_OF_VIEW: f64 = consts::FRAC_PI_2 * 1.2;

fn camera() -> Camera {
    let from = Tup::point(0.0, -0.8, -5.0);
    let to = Tup::point(0.0, 0.0, 0.0);
    let up = Tup::vector(0.0, 1.0, 0.0);
    let camera_transform = transforms::view_transform(from, to, up);
    Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, CAMERA_FIELD_OF_VIEW)
        .with_transform(camera_transform)
        .with_progress_logging()
}

fn sphere(x: f64, y: f64, z: f64, color: Color) -> Sphere {
    let radius = 0.55;
    let ambient = 0.2;
    let diffuse = 0.7;
    let specular = 0.3;
    let reflective = 0.2;
    let material = Material::default()
        .with_color(color)
        .with_diffuse(diffuse)
        .with_reflective(reflective)
        .with_ambient(ambient)
        .with_specular(specular);

    Sphere::default()
        .with_transform(transforms::scaling(radius, radius, radius))
        .with_transform(transforms::translation(x, y, z))
        .with_material(material)
}

fn single_sphere() -> Canvas {
    let camera = camera();
    let mut world = World::default()
        .with_light(light_source())
        .with_object(sphere(0.0, 0.0, 0.0, col::RED))
        .with_object(sphere(-2.5, 0.0, 2.5, col::BLUE))
        .with_object(sphere(2.5, 0.0, 2.0, col::GREEN))
        .with_object(sphere(0.0, -2.5, 0.0, col::MAGENTA))
        .with_object(sphere(0.0, 2.5, 0.0, col::CYAN));
    camera.render(&world)
}

fn light_source() -> Light {
    let light_position = Tup::point(-10, 10, -10);
    let light_intensity = Color::new(1, 1, 1);
    Light::point_light(light_position, light_intensity)
}

fn main() -> std::io::Result<()> {
    let image_name = "sphere";
    let canvas = single_sphere();
    let pixels = canvas.to_p6_ppm();
    let file_name = format!("{}.ppm", image_name);
    fs::write(file_name, pixels)?;
    Ok(())
}
