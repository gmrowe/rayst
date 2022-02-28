use crate::matrix::Mat4;
use crate::rays::Ray;
use crate::tup::Tup;
use crate::world::World;
use crate::canvas::Canvas;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Mat4,
    log_progress: bool,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Mat4::identity_matrix(),
            log_progress: false,
        }
    }

    pub fn with_transform(self, transform: Mat4) -> Self {
        Self {
            transform,
            ..self
        }
    }

    pub fn with_progress_logging(self) -> Self {
        Self {
            log_progress: true,
            ..self
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn field_of_view(&self) -> f64 {
        self.field_of_view
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    fn half_width_and_height(&self) -> (f64, f64) {
        let half_view = (self.field_of_view() / 2.0).tan();
        let aspect_ratio = self.hsize() as f64 / self.vsize() as f64;
        if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        }
    }

    pub fn pixel_size(&self) -> f64 {
        let (half_width, _) = self.half_width_and_height();
        half_width * 2.0 / self.hsize as f64
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let pixel_size = self.pixel_size();
        let x_offset = (px as f64 + 0.5) * pixel_size;
        let y_offset = (py as f64 + 0.5) * pixel_size;
        let (half_width, half_height) = self.half_width_and_height();
        let world_x = half_width - x_offset;
        let world_y = half_height - y_offset;
        let inverse_transform = self.transform.inverse();
        let pixel = inverse_transform * Tup::point(world_x, world_y, -1.0);
        let origin = inverse_transform * Tup::point(0, 0, 0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    fn output_progress(&self, row: usize, col: usize) {
        let pixel_count = (self.hsize * self.vsize) as f64;
        let pixel_number = (row * self.hsize + col) as f64;
        let percent_complete = pixel_number / pixel_count * 100.0;
        print!("{:.0}% complete\r", percent_complete); 
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for (row, col, pixel) in image.enumerate_pixels_mut() {
            let ray = self.ray_for_pixel(col, row);
            let color = world.color_at(ray);
            *pixel = color;
            
            if self.log_progress {
                self.output_progress(row, col);
            }
        }
        image
    }
}

#[cfg(test)]
mod camera_test {
    use super::*;
    use std::f64::consts;
    use crate::transforms;
    use crate::lights::Light;
    use crate::color::Color;
    use crate::materials::Material;
    use crate::spheres::Sphere;
    use crate::test_helpers::assert_nearly_eq;
    
    #[test]
    fn a_camera_stores_its_hsize() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(hsize, camera.hsize());
    }

    #[test]
    fn a_camera_stores_its_vsize() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(vsize, camera.vsize());
    }

    #[test]
    fn a_camera_stores_its_field_of_view() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(field_of_view, camera.field_of_view());
    }

    #[test]
    fn a_camera_stores_its_transform() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(Mat4::identity_matrix(), camera.transform());
    }

    #[test]
    fn camera_knows_pixel_size_for_horizontal_canvas() {
        let hsize = 200;
        let vsize = 125;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_nearly_eq(0.01, camera.pixel_size());
    }

    #[test]
    fn camera_knows_pixel_size_for_vertical_canvas() {
        let hsize = 125;
        let vsize = 200;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_nearly_eq(0.01, camera.pixel_size());
    }

    #[test]
    fn a_ray_through_the_center_of_the_canvas_can_be_calculated() {
        let hsize = 201;
        let vsize = 101;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(Tup::point(0, 0, 0), r.origin());
        assert_eq!(Tup::vector(0, 0, -1), r.direction());
    }

    #[test]
    fn a_ray_through_a_corner_of_the_canvas_can_be_calculated() {
        let hsize = 201;
        let vsize = 101;
        let field_of_view = consts::PI / 2.0;
        let camera = Camera::new(hsize, vsize, field_of_view);
        let r = camera.ray_for_pixel(0, 0);
        assert_eq!(Tup::point(0, 0, 0), r.origin());
        assert_eq!(Tup::vector(0.66519, 0.33259, -0.66851), r.direction());
    }

    #[test]
    fn a_ray_can_be_calculated_when_camera_is_transfomed() {
        let hsize = 201;
        let vsize = 101;
        let field_of_view = consts::PI / 2.0;
        let rotation = transforms::rotation_y(consts::PI / 4.0);
        let translation = transforms::translation(0, -2, 5);
        let transform = rotation * translation;
        let camera = Camera::new(hsize, vsize, field_of_view).with_transform(transform);
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(Tup::point(0, 2, -5), r.origin());
        let sqrt_2_over_2 = 2.0_f64.sqrt() / 2.0;
        assert_eq!(Tup::vector(sqrt_2_over_2, 0.0, -sqrt_2_over_2), r.direction());
    }

    
    fn default_test_world() -> World {
        let light =
            Light::point_light(Tup::point(-10, 10, -10), Color::new(1, 1, 1));
        
        let material = Material::default()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Sphere::default().with_material(material);
        
        let transform = transforms::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::default().with_transform(transform);
        
        World::default()
            .with_light(light)
            .with_object(s1)
            .with_object(s2)
    }

    #[test]
    fn a_world_can_be_rendered_from_a_camera() {
        let world = default_test_world();
        let from = Tup::point(0, 0, -5);
        let to = Tup::point(0, 0, 0);
        let up = Tup::vector(0, 1, 0);
        let transform = transforms::view_transform(from, to, up);
        let camera = Camera::new(11, 11, consts::PI/2.0).with_transform(transform);
        let image = camera.render(&world);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855))
    }
}
