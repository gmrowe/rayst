mod canvas;
mod color;
mod math_helpers;
mod tup;
mod matrix;

use tup::Tup;
use canvas::Canvas;
use color::Color;
use std::fs;

fn plot(point: &Tup, c: Canvas, color: Color) -> Canvas {
    let x = point.x.round() as usize;
    let y = c.height() - point.y.round() as usize;
    if x < c.width() && y < c.height() {
        c.write_pixel(x, y, color)
    } else {
        c
    }
}

//fn write_to_file()

fn main() -> std::io::Result<()> {
    use std::iter;
    let start = Tup::point(0.0, 1.0, 0.0);
    let velocity = Tup::vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let projectile = Projectile::new(start, velocity);

    let gravity = Tup::vector(0.0, -0.1, 0.0);
    let wind = Tup::vector(-0.01, 0.0, 0.0);
    let environment = Environment::new(gravity, wind);

    const WIDTH: usize = 900;
    const HEIGHT: usize = 550;
    let red = Color::new(1.0, 0.0, 0.0);

    let image = iter::successors(Some(projectile), |p| Some(environment.tick(p)))
        .take_while(|p| p.pos.y > 0.0)
        .fold(Canvas::new(WIDTH, HEIGHT), |c, p| plot(&p.pos, c, red))
        .to_ppm();

    fs::write("projectile.ppm", image)?;
    Ok(())
}

struct Projectile {
    pos: Tup, // Point
    vel: Tup, // Vector
}

impl Projectile {
    fn new(pos: Tup, vel: Tup) -> Self {
        assert!(pos.is_point(), "Position must be a point");
        assert!(vel.is_vector(), "Velocity must be a vector");
        Self {
            pos, vel
        }
    }
}

struct Environment {
    gravity: Tup, // Vector
    wind: Tup,    // Vector 
}

impl Environment {
    fn new(gravity: Tup, wind: Tup) -> Self {
        assert!(gravity.is_vector(), "Gravity must be a vector");
        assert!(wind.is_vector(), "Wind must be a vector");
        Self {
            gravity, wind
        }
    }
    
    fn tick(&self, proj: &Projectile) -> Projectile {
        let new_pos = proj.pos + proj.vel;
        let new_vel = proj.vel + self.gravity + self.wind;
        Projectile::new(new_pos, new_vel)
    }
}
