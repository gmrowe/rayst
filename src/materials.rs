use crate::Color;

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
}
