mod canvas;
mod color;
mod math_helpers;
mod tup;

use tup::Tup;
use canvas::Canvas;
use color::Color;
use std::fs::write;

fn plot(point: &Tup, c: Canvas, color: Color) -> Canvas {
    let x = point.x.round() as usize;
    let y = c.height() - point.y.round() as usize;
    if x < c.width() && y < c.height() {
        c.write_pixel(x, y, color)
    } else {
        c
    }
}

fn main() -> std::io::Result<()> {
    use std::iter::successors;
    let start = Tup::point(0.0, 1.0, 0.0);
    let velocity = Tup::vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let projectile = Projectile::new(start, velocity);

    let gravity = Tup::vector(0.0, -0.1, 0.0);
    let wind = Tup::vector(-0.01, 0.0, 0.0);
    let environment = Environment::new(gravity, wind);

    const WIDTH: usize = 900;
    const HEIGHT: usize = 550;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let red = Color::new(1.0, 0.0, 0.0);

    let flight = successors(Some(projectile), |p| Some(environment.tick(p)))
        .take_while(|p| p.pos.y > 0.0);

    for p in flight {
        canvas = plot(&p.pos, canvas, red);
    }
    let ppm = canvas.to_ppm();
    write("projectile.ppm", ppm)?;
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
