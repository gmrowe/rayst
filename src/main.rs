mod camera;
mod canvas;
mod color;
mod intersections;
mod lights;
mod materials;
mod math_helpers;
mod matrix;
mod rays;
mod patterns;
mod planes;
mod shapes;
mod spheres;
mod transforms;
mod test_helpers;
mod tup;
mod world;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::color::consts as col;
use crate::planes::Plane;
use crate::lights::Light;
use crate::materials::Material;
use crate::tup::Tup;
use crate::patterns::Pattern;
use crate::spheres::Sphere;
use crate::world::World;
use std::fs;
use std::f64::consts;

fn background_material() -> Material {
    Material::default()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0)
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


fn middle_sphere() -> Sphere {
    let translation = transforms::translation(0.0, 0.85, -0.12);
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

fn light_source() -> Light {
    let light_position = Tup::point(-10, 10, -10);
    let light_intensity = Color::new(1, 1, 1);
    Light::point_light(light_position, light_intensity)
}

fn plane_floor()-> Plane {
    const FLOOR_SPECULAR: f64 = 0.3;
    const FLOOR_SHININESS: f64 = 200.0;
    let transform = transforms::scaling(4, 4, 4) * transforms::rotation_y(consts::PI/4.0);
    let pattern = Pattern::gradient_pattern(col::RED, col::GREEN)
        .with_transform(transform);
        
    let material = Material::default()
        .with_specular(FLOOR_SPECULAR)
        .with_shininess(FLOOR_SHININESS)
        .with_pattern(pattern);
    
    Plane::default()
        .with_material(material)
}

fn back_wall() -> Plane {
    const WALL_SPECULAR: f64 = 0.3;
    const WALL_SHININESS: f64 = 10.0;
    let wall_color = col::MAGENTA;
    
    let material = Material::default()
        .with_specular(WALL_SPECULAR)
        .with_shininess(WALL_SHININESS)
        .with_color(wall_color);

    let rot_x = transforms::rotation_x(consts::PI/2.0);
    let translate = transforms::translation(0, 0, 10);
    
    Plane::default()
        .with_material(material)
        .with_transform(translate * rot_x)
}

fn plane_wall_color() -> Color {
    col::CYAN + (col::GREEN * 0.25)
}

fn left_plane_wall() -> Plane {
    let translation = transforms::translation(0.0, 0.0, 10.0);
    let rot_y = transforms::rotation_y(-consts::PI/4.0);
    let rot_x = transforms::rotation_x(consts::PI/2.0);
    let left_wall_transform = translation * rot_y * rot_x;
    let pattern = Pattern::checkers_pattern(col::BLACK, col::GREEN)
        .with_transform(transforms::scaling(0.4, 0.1, 1.0));

    Plane::default()
        .with_transform(left_wall_transform)
        .with_material(background_material().with_pattern(pattern))
}

fn right_plane_wall() -> Plane {
    let translation = transforms::translation(0.0, 0.0, 10.0);
    let rot_y = transforms::rotation_y(consts::PI/4.0);
    let rot_x = transforms::rotation_x(consts::PI/2.0);
    let right_wall_transform = translation * rot_y * rot_x;
    let pattern = Pattern::ring_pattern(col::CYAN, col::MAGENTA)
        .with_transform(transforms::scaling(0.1, 0.1, 0.1));

    Plane::default()
        .with_transform(right_wall_transform)
        .with_material(background_material().with_pattern(pattern))
}


fn spheres_in_a_corner() -> Canvas {
    let camera = camera();
    let world = World::default()
        .with_light(light_source())
        .with_object(plane_floor())
        .with_object(right_plane_wall())
        .with_object(left_plane_wall())
        .with_object(left_sphere())
        .with_object(right_sphere())
        .with_object(middle_sphere());

    camera.render(&world)
}

fn main() -> std::io::Result<()> {
    let image_name = "spheres_in_a_corner";
    let canvas = spheres_in_a_corner();
    let pixels = canvas.to_p6_ppm();
    let file_name = format!("{}.ppm", image_name);
    fs::write(file_name, pixels)?;
    Ok(())
}

