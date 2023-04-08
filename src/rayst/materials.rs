use crate::rayst::color::consts;
use crate::rayst::color::Color;
use crate::rayst::lights::Light;
use crate::rayst::matrix::Mat4;
use crate::rayst::patterns::Pattern;
use crate::rayst::tup::Tup;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Material {
    color: Color,
    // Ambient reflection is background lighting or light reflected from
    // other objects in the environment. THis is treated as constant, coloring
    // all points equqlly
    ambient: f64,

    // Diffuse reflection is light reflected from a matte surface. It depends on
    // the angle between the light source and the surface normal
    diffuse: f64,

    // Specular reflection is the reflection of the light source which causes
    // a `specular highlight` that is often manifest as a white spot on a shiny
    // surface
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    pattern: Option<Pattern>,
}

impl Material {
    pub fn with_ambient(self, ambient: f64) -> Self {
        Self { ambient, ..self }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn with_diffuse(self, diffuse: f64) -> Self {
        Self { diffuse, ..self }
    }

    pub fn with_specular(self, specular: f64) -> Self {
        Self { specular, ..self }
    }

    pub fn with_shininess(self, shininess: f64) -> Self {
        Self { shininess, ..self }
    }

    pub fn with_pattern(self, pattern: Pattern) -> Self {
        Self {
            pattern: Some(pattern),
            ..self
        }
    }

    pub fn with_reflective(self, reflective: f64) -> Self {
        Self { reflective, ..self }
    }

    pub fn with_transparency(self, transparency: f64) -> Self {
        Self {
            transparency,
            ..self
        }
    }

    pub fn with_refractive_index(self, refractive_index: f64) -> Self {
        Self {
            refractive_index,
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

    pub fn reflective(&self) -> f64 {
        self.reflective
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    fn calc_diffuse(&self, effective_color: Color, light_dot_normal: f64) -> Color {
        effective_color * self.diffuse() * light_dot_normal
    }

    fn calc_specular(&self, lightv: Tup, normalv: Tup, eyev: Tup, light: Light) -> Color {
        let reflectv = -lightv.reflect(&normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);
        if reflect_dot_eye <= 0.0 {
            consts::BLACK
        } else {
            let factor = reflect_dot_eye.powf(self.shininess());
            light.intensity() * self.specular() * factor
        }
    }

    pub fn lighting(
        &self,
        object_transform: Mat4,
        light: Light,
        position: Tup,
        eyev: Tup,
        normalv: Tup,
        in_shadow: bool,
    ) -> Color {
        let color = self
            .pattern
            .map(|p| p.color_at(object_transform, position))
            .unwrap_or(self.color);
        let effective_color = color * light.intensity();
        let lightv = (light.position() - position).normalize();
        let ambient = effective_color * self.ambient();
        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular) = if light_dot_normal < 0.0 || in_shadow {
            (consts::BLACK, consts::BLACK)
        } else {
            (
                self.calc_diffuse(effective_color, light_dot_normal),
                self.calc_specular(lightv, normalv, eyev, light),
            )
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
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }
}

#[cfg(test)]
mod materials_test {
    use super::*;
    use crate::rayst::color::consts as color;

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
        let result = m.lighting(Mat4::default(), light, position, eyev, normalv, false);
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
        let eyev = Tup::vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(Mat4::default(), light, position, eyev, normalv, false);
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
        let result = m.lighting(Mat4::default(), light, position, eyev, normalv, false);
        let sum_of_lights =
            m.ambient() + (2.0_f64.sqrt() / 2.0 * m.diffuse()) + (0.0 * m.specular());
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(Mat4::default(), light, position, eyev, normalv, false);
        let sum_of_lights = m.ambient() + (2.0_f64.sqrt() / 2.0 * m.diffuse()) + m.specular();
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
        let result = m.lighting(Mat4::default(), light, position, eyev, normalv, false);
        let sum_of_lights = m.ambient() + (0.0 * m.diffuse()) + (0.0 * m.specular());
        assert_eq!(
            Color::new(sum_of_lights, sum_of_lights, sum_of_lights),
            result
        );
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 0, -10), Color::new(1, 1, 1));
        let in_shadow = true;
        let result = m.lighting(Mat4::default(), light, position, eyev, normalv, in_shadow);
        assert_eq!(Color::new(m.ambient(), m.ambient(), m.ambient()), result);
    }

    #[test]
    fn lighting_a_material_with_a_pattern() {
        let m = Material::default()
            .with_pattern(Pattern::stripe_pattern(color::WHITE, color::BLACK))
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::point_light(Tup::point(0, 0, -10), color::WHITE);
        let in_shadow = false;
        let c1 = m.lighting(
            Mat4::default(),
            light,
            Tup::point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            in_shadow,
        );
        let c2 = m.lighting(
            Mat4::default(),
            light,
            Tup::point(1.1, 0.0, 0.0),
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(color::WHITE, c1);
        assert_eq!(color::BLACK, c2)
    }

    #[test]
    fn material_has_a_default_reflectivity() {
        let m = Material::default();
        assert_eq!(0.0, m.reflective());
    }

    #[test]
    fn material_has_a_default_transparancy() {
        let m = Material::default();
        assert_eq!(0.0, m.transparency());
    }

    #[test]
    fn material_transparancy_is_settable() {
        let m = Material::default().with_transparency(0.5);
        assert_eq!(0.5, m.transparency());
    }

    #[test]
    fn material_has_a_default_refractive_index() {
        let m = Material::default();
        assert_eq!(1.0, m.refractive_index());
    }

    #[test]
    fn material_refractive_index_is_settable() {
        let m = Material::default().with_refractive_index(1.8);
        assert_eq!(1.8, m.refractive_index());
    }
}
