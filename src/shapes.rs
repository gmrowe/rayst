use crate::matrix::Mat4;
use crate::intersections::Intersections;
use crate::materials::Material;
use crate::rays::Ray;
use crate::tup::Tup;

trait Shape {
    fn transform(&self) -> Mat4;

    fn set_transform(&mut self, transform: Mat4);

    fn material(&self) -> Material;

    fn set_material(&mut self, material: Material);
    
    fn intersect(&self, ray: Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform().inverse());
        self.local_intersect(local_ray)
    }
    
    fn local_intersect(&self, local_ray: Ray) -> Intersections;

    fn normal_at(&self, point: Tup) -> Tup {
        let inverse_xform = self.transform().inverse();
        let local_point = inverse_xform * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = inverse_xform.transpose() * local_normal;
        // Hack to ensure that w = 1.0 - See pg. 82
        let world_normal_vec =
            Tup::vector(world_normal.x, world_normal.y, world_normal.z);
        world_normal_vec.normalize()
    }

    fn local_normal_at(&self, point: Tup) -> Tup;
}

#[cfg(test)]
mod shape_tests {
    use super::*;
    use crate::transforms;
    use core::f64::consts;

    static mut SAVED_RAY: Option<Ray> = None;

    struct TestShape {
        transform: Option<Mat4>,
        material: Option<Material>,
    }

    impl Default for TestShape {
        fn default() -> Self {
            Self {
                transform: None,
                material: None,
            }
        }
    }

    impl Shape for TestShape {
        fn transform(&self) -> Mat4 {
            self.transform.unwrap_or_default()
        }

        fn set_transform(&mut self, transform: Mat4) {
            self.transform  = Some(transform);
        }

        fn material(&self) -> Material {
            self.material.unwrap_or_default()
        }

        fn set_material(&mut self, material: Material) {
            self.material = Some(material);
        }

        fn local_intersect(&self, local_ray: Ray) -> Intersections {
            unsafe {
                SAVED_RAY = Some(local_ray);
            }
            Intersections::default()
        }

        fn local_normal_at(&self, point: Tup) -> Tup {
            Tup::vector(point.x, point.y, point.z)
        }
    }
    
    #[test]
    fn shape_should_have_a_default_transformation() {
        let shape = TestShape::default();
        let transform = shape.transform();
        assert_eq!(Mat4::identity_matrix(), transform);
    }

    #[test]
    fn a_transform_should_be_assignable_to_a_shape() {
        let mut shape = TestShape::default();
        shape.set_transform(transforms::translation(2, 3, 4));
        assert_eq!(transforms::translation(2, 3, 4), shape.transform());
    }

    #[test]
    fn a_shape_should_have_a_default_material() {
        let shape = TestShape::default();
        assert_eq!(Material::default(), shape.material());
    }

    #[test]
    fn a_material_should_be_assignable_to_a_shape() {
        let mut shape = TestShape::default();
        let material = Material::default().with_ambient(1.0);
        shape.set_material(material);
        assert_eq!(material, shape.material());
    }

    #[test]
    fn a_scaled_shape_can_intersect_with_a_ray() {
        let ray = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let mut shape = TestShape::default();
        shape.set_transform(transforms::scaling(2, 2, 2));
        let _xs = shape.intersect(ray);
        unsafe {
            assert_eq!(Tup::point(0.0, 0.0, -2.5), SAVED_RAY.expect("No saved ray").origin());
            assert_eq!(Tup::vector(0.0, 0.0, 0.5), SAVED_RAY.expect("No saved ray").direction());
        }
    }

    #[test]
    fn a_translated_shape_can_intersect_with_a_ray() {
        let ray = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let mut shape = TestShape::default();
        shape.set_transform(transforms::translation(5, 0, 0));
        let _xs = shape.intersect(ray);
        unsafe {
            assert_eq!(Tup::point(-5, 0, -5), SAVED_RAY.expect("No saved ray").origin());
            assert_eq!(Tup::vector(0.0, 0.0, 1.0), SAVED_RAY.expect("No saved ray").direction());
        }
    }

    #[test]
    fn the_normal_on_a_translated_shape_can_be_calculates() {
        let mut shape = TestShape::default();
        shape.set_transform(transforms::translation(0, 1, 0));
        let n = shape.normal_at(Tup::point(0.0, 1.70711, -0.70711));
        assert_eq!(Tup::vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn the_normal_on_a_tranformed_shape_can_be_calculates() {
        let mut shape = TestShape::default();
        let transform = transforms::scaling(1.0, 0.5, 1.0) * transforms::rotation_z(consts::PI/5.0);
        shape.set_transform(transform);
        let n = shape.normal_at(Tup::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0, ));
        assert_eq!(Tup::vector(0.0, 0.97014, -0.24254), n);
    } 
}
