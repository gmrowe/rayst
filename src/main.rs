mod tup;

use tup::Tup;

fn main() {
    use std::iter::successors;
    let projectile = Projectile::new(
        Tup::point(0.0, 1.0, 0.0),
        Tup::vector(1.0, 1.0, 0.0).normalize()
    );

    let environment = Environment::new(
        Tup::vector(0.0, -0.1, 0.0),
        Tup::vector(-0.02, 0.01, 0.0),
    );

    let final_pos =
        successors(Some(projectile), |p| Some(environment.tick(p)))
        .take_while(|p| p.pos.y > 0.0)
        .last()
        .expect("No y values > 0")
        .pos;

    println!("Final position: {:?}", final_pos);
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
