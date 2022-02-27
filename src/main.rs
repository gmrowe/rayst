mod camera;
mod canvas;
mod color;
mod intersections;
mod lights;
mod materials;
mod math_helpers;
mod matrix;
mod rays;
mod spheres;
mod transforms;
mod tup;
mod world;

use canvas::Canvas;
use color::Color;
use std::fs;
use std::f64::consts;
use lights::Light;
use materials::Material;
use tup::Tup;
use spheres::Sphere;
use rays::Ray;

fn plot(point: &Tup, c: Canvas, color: &Color) -> Canvas {
    let x = point.x.round() as usize;
    let y = c.height() - point.y.round() as usize;
    if x < c.width() && y < c.height() {
        c.write_pixel(x, y, *color)
    } else {
        c
    }
}

fn chapter_4_putting_it_together_clock() -> String {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let white = Color::new(1.0, 1.0, 1.0);
    const NUM_VALUES: usize = 12;
    for i in 0..NUM_VALUES {
        let twelve = Tup::point(0.0, 1.0, 0.0);
        let coord_trans =
            transforms::translation(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0, 0.0);
        let scale =
            transforms::scaling(HEIGHT as f64 / 3.0, HEIGHT as f64 / 3.0, 0.0);
        let z_rot =
             transforms::rotation_z((consts::PI * 2.0) / NUM_VALUES as f64 * i as f64);
        let translate = coord_trans * scale * z_rot;
        let p = translate * twelve;
        canvas = plot(&p, canvas, &white);
    }
    canvas.to_ppm()
}

fn chapter_5_putting_it_together() -> String {
    let camera = Tup::point(0, 0, -20);
    let mut sphere = Sphere::default();
    const WALL_WIDTH: f64 = 20.0;
    const WALL_HEIGHT: f64 = 20.0;
    const CANVAS_WIDTH: usize = 200;
    const CANVAS_HEIGHT: usize = 200;
    const CANVAS_DISTANCE: f64 = 1.0;
    const PIXEL_WIDTH: f64 = WALL_WIDTH / CANVAS_WIDTH as f64;
    const PIXEL_HEIGHT: f64 = WALL_HEIGHT / CANVAS_HEIGHT as f64;
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let scale = transforms::scaling(2, 2, 2);
    let trans = transforms::translation(-2, 8, 0);
    sphere = sphere.set_transform(trans * scale);
    let red = Color::new(1.0, 0.0, 0.0);
    
    for (row, col, pixel) in canvas.enumerate_pixels_mut() {
        let x = (col as f64 * PIXEL_WIDTH) - (WALL_WIDTH / 2.0);
        let y = (WALL_HEIGHT / 2.0) - (row as f64 * PIXEL_HEIGHT);
        let z = CANVAS_DISTANCE;
        let vec = (Tup::point(x, y, z) - camera).normalize();
        let ray = Ray::new(camera, vec);
        let xs = sphere.intersect(&ray);
        if xs.hit().is_some() {
            *pixel = red;
        }
    }
    canvas.to_ppm()
}

fn chapter_6_putting_it_together() -> Vec<u8> {
    let sphere_material = Material::default()
        .with_color(Color::from_hex(0x324AB2))
        .with_ambient(0.10);
    let sphere = Sphere::default()
        .with_material(sphere_material)
        .with_transform(transforms::scaling(2, 2, 1))
        ;
    let camera = Tup::point(0, 0, -5);
    let light_pos = Tup::point(-10, 10, -10);
    let light_color = Color::new(1, 1, 1);
    let light = Light::point_light(light_pos, light_color);
    const WALL_WIDTH: f64 = 10.0;
    const WALL_HEIGHT: f64 = 10.0;
    const CANVAS_WIDTH: usize = 600;
    const CANVAS_HEIGHT: usize = 600;
    const CANVAS_DISTANCE: f64 = 1.0;
    const PIXEL_WIDTH: f64 = WALL_WIDTH / CANVAS_WIDTH as f64;
    const PIXEL_HEIGHT: f64 = WALL_HEIGHT / CANVAS_HEIGHT as f64;
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    const PIXEL_COUNT: usize = CANVAS_WIDTH * CANVAS_HEIGHT;
       
    for (row, col, pixel) in canvas.enumerate_pixels_mut() {
        let x = (col as f64 * PIXEL_WIDTH) - (WALL_WIDTH / 2.0);
        let y = (WALL_HEIGHT / 2.0) - (row as f64 * PIXEL_HEIGHT);
        let z = CANVAS_DISTANCE;
        let vec = (Tup::point(x, y, z) - camera).normalize();
        let ray = Ray::new(camera, vec);
        let xs = sphere.intersect(&ray);
        if let Some(intersection) = xs.hit() {
            let hit_object = intersection.object();
            let point = ray.position(intersection.t());
            let normal = hit_object.normal_at(point);
            let eyev = -ray.direction();
            let color = hit_object.material().lighting(light, point, eyev, normal);
            *pixel = color;
        }
        let percent_complete = (row * CANVAS_WIDTH + col) as f64 / PIXEL_COUNT  as f64 * 100.0;
        print!("{:.0}% complete\r", percent_complete);
    }
    println!();
    canvas.to_p6_ppm()
}

fn main() -> std::io::Result<()> {
    println!("Computing pixels");
    let pixels = chapter_6_putting_it_together();
    println!("Writing file");
    fs::write("sphere_bin.ppm", pixels)?;
    Ok(())
}

