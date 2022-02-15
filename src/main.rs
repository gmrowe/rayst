mod canvas;
mod color;
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

fn plot(point: &Tup, c: Canvas, color: &Color) -> Canvas {
    let x = point.x.round() as usize;
    let y = c.height() - point.y.round() as usize;
    if x < c.width() && y < c.height() {
        c.write_pixel(x, y, *color)
    } else {
        c
    }
}

//fn write_to_file()

fn main() -> std::io::Result<()> {
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

    let image = canvas.to_ppm();
    fs::write("clock.ppm", image)?;
    Ok(())
}

