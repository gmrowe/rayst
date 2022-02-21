use crate::color::Color;
use crate::tup::Tup;
use crate::lights::Light;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn with_ambient(self, ambient: f64) -> Self {
        Self {
            ambient,
            ..self
        }
    }
    
    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn lighting(
        &self,
        light: Light,
        position: Tup,
        eyev: Tup,
        normalv: Tup
    ) -> Color
    {
        let black = Color::new(0, 0, 0);
        let effective_color = self.color() * light.intensity();
        let lightv = (light.position() - position).normalize();
        let ambient = effective_color * self.ambient();
        let light_dot_normal = lightv.dot(&normalv);
        let diffuse = if light_dot_normal < 0.0 {
            black
        } else {
            effective_color * self.diffuse() * light_dot_normal
        };
        let specular = if light_dot_normal < 0.0 {
            black
        } else {
            let reflectv = -lightv.reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye <= 0.0 {
                black
            } else {
                let factor = reflect_dot_eye.powf(self.shininess());
                light.intensity() * self.specular() * factor
            }
        };
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            color: Color::new(1, 1, 1),
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}


mod materials_test {
    use super::*;

    #[test]
    fn default_material_has_a_color() {
        let m = Material::default();
        assert_eq!(Color::new(1, 1, 1), m.color());
    }
    
    #[test]
    fn default_material_has_ambient_reflection() {
        let m = Material::default();
        assert_eq!(0.1, m.ambient());
    }

    #[test]
    fn default_material_has_diffuse_reflection() {
        let m = Material::default();
        assert_eq!(0.9, m.diffuse());
    }

    #[test]
    fn default_material_has_specular_reflection() {
        let m = Material::default();
        assert_eq!(0.9, m.specular());
    }

    #[test]
    fn default_material_has_shininess() {
        let m = Material::default();
        assert_eq!(200.0, m.shininess());
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        let sum_of_lights = m.ambient() + m.diffuse() + m.specular();
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        let sum_of_lights = m.ambient() + m.diffuse() + (0.0 * m.specular());
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        let sum_of_lights = m.ambient()
            + (2.0_f64.sqrt()/2.0 * m.diffuse())
            + (0.0 * m.specular());
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0.0, -2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        let sum_of_lights = m.ambient()
            + (2.0_f64.sqrt()/2.0 * m.diffuse())
            + m.specular();
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, 1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 0, 10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        let sum_of_lights = m.ambient()
            + (0.0 * m.diffuse())
            + (0.0 * m.specular());
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }
}
