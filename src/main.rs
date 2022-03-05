mod camera;
mod canvas;
mod color;
mod intersections;
mod lights;
mod materials;
mod math_helpers;
mod matrix;
mod rays;
mod planes;
mod shapes;
mod spheres;
mod transforms;
mod test_helpers;
mod tup;
mod world;

use camera::Camera;
use canvas::Canvas;
use color::Color;
use std::fs;
use std::f64::consts;
use lights::Light;
use materials::Material;
use tup::Tup;
use spheres::Sphere;
use world::World;


fn background_material() -> Material {
    Material::default()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0)
}

fn floor() -> Sphere {
    let floor_transform = transforms::scaling(10.0, 0.01, 10.0);
    
    Sphere::default()
        .with_transform(floor_transform)
        .with_material(background_material())
}

fn left_wall() -> Sphere {
    let translation = transforms::translation(0.0, 0.0, 5.0);
    let rot_y = transforms::rotation_y(-consts::PI/4.0);
    let rot_x = transforms::rotation_x(consts::PI/2.0);
    let scaling = transforms::scaling(10.0, 0.01, 10.0);
    let left_wall_transform = translation * rot_y * rot_x * scaling;

    Sphere::default()
        .with_transform(left_wall_transform)
        .with_material(background_material())
}

fn right_wall() -> Sphere {
    let translation = transforms::translation(0.0, 0.0, 5.0);
    let rot_y = transforms::rotation_y(consts::PI/4.0);
    let rot_x = transforms::rotation_x(consts::PI/2.0);
    let scaling = transforms::scaling(10.0, 0.01, 10.0);
    let right_wall_transform = translation * rot_y * rot_x * scaling;

    Sphere::default()
        .with_transform(right_wall_transform)
        .with_material(background_material())
}

fn middle_sphere() -> Sphere {
    let translation = transforms::translation(-0.5, 1.0, 0.5);
    let color = Color::from_hex(0x8C11D9);
    let diffuse = 0.7;
    let specular = 0.3;
    let material = Material::default()
        .with_color(color)
        .with_diffuse(diffuse)
        .with_specular(specular);

    Sphere::default()
        .with_transform(translation)
        .with_material(material)
}

fn right_sphere() -> Sphere {
    let translation = transforms::translation(1.5, 0.5, -0.5);
    let scaling = transforms::scaling(0.5, 0.5, 0.5);
    let transform = translation * scaling;
    let color = Color::new(0.5, 1.0, 0.1);
    let diffuse = 0.7;
    let specular = 0.3;
    let material = Material::default()
        .with_color(color)
        .with_diffuse(diffuse)
        .with_specular(specular);

    Sphere::default()
        .with_transform(transform)
        .with_material(material)
}

fn left_sphere() -> Sphere {
    let translation = transforms::translation(-1.5, 0.33, -0.75);
    let scaling = transforms::scaling(0.33, 0.33, 0.33);
    let transform = translation * scaling;
    let color = Color::from_hex(0xE8D34D);
    let diffuse = 0.7;
    let specular = 0.3;
    let material = Material::default()
        .with_color(color)
        .with_diffuse(diffuse)
        .with_specular(specular);

    Sphere::default()
        .with_transform(transform)
        .with_material(material)
}

fn camera() -> Camera {
    const CANVAS_WIDTH: usize = 1200;
    const CANVAS_HEIGHT: usize = 600;
    const CAMERA_FIELD_OF_VIEW: f64 = consts::PI / 3.0;
    let from = Tup::point(0.0, 1.5, -5.0);
    let to = Tup::point(0.0, 1.0, 0.0);
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

fn chapter_7_putting_it_together() -> Canvas {
    let camera = camera();
    let world = World::default()
        .with_light(light_source())
        .with_object(floor())
        .with_object(left_wall())
        .with_object(right_wall())
        .with_object(middle_sphere())
        .with_object(right_sphere())
        .with_object(left_sphere());
    
    camera.render(&world)
}

fn main() -> std::io::Result<()> {
    let canvas = chapter_7_putting_it_together();
    let pixels = canvas.to_p6_ppm();
    fs::write("scene.ppm", pixels)?;
    Ok(())
}

