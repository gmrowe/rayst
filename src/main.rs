mod canvas;
mod color;
mod intersections;
mod math_helpers;
mod matrix;
mod rays;
mod spheres;
mod transforms;
mod tup;

use canvas::Canvas;
use color::Color;
use std::fs;
use std::f64::consts;
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
    let camera = Tup::point(0, 0, -5);
    let mut sphere = Sphere::default();
    const CANVAS_WIDTH: usize = 200;
    const CANVAS_HEIGHT: usize = 200;
    const CANVAS_DISTANCE: f64 = 1.0;
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let scale = transforms::scaling(30, 10, 1);
    sphere = sphere.set_transform(scale);
    let red = Color::new(1.0, 0.0, 0.0);
    
    for (row, col, pixel) in canvas.enumerate_pixels_mut() {
        let x = col as f64 - (CANVAS_WIDTH / 2)  as f64;
        let y = (CANVAS_HEIGHT / 2) as f64 - row as f64;
        let z = CANVAS_DISTANCE;
        let vec = (Tup::point(x, y, z) - camera).normalize();
        let ray = Ray::new(camera, vec);
        let xs = sphere.intersect(&ray);
        if let Some(_) = xs.hit() {
            *pixel = red;
        }
    }
    canvas.to_ppm()
}

fn main() -> std::io::Result<()> {
    fs::write("shadow.ppm", chapter_5_putting_it_together())?;
    Ok(())
}

