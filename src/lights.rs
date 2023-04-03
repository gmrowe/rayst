use crate::Color;
use crate::Tup;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Light {
    position: Tup,
    intensity: Color,
}

impl Light {
    pub fn point_light(position: Tup, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> Tup {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod lights_test {
    use super::*;

    #[test]
    fn a_point_light_has_position() {
        let intensity = Color::new(1, 1, 1);
        let position = Tup::point(0, 0, 0);
        let point_light = Light::point_light(position, intensity);
        assert_eq!(position, point_light.position());
    }

    #[test]
    fn a_point_light_has_intensity() {
        let intensity = Color::new(1, 1, 1);
        let position = Tup::point(0, 0, 0);
        let point_light = Light::point_light(position, intensity);
        assert_eq!(intensity, point_light.intensity());
    }
}
