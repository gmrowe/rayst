use crate::matrix::Mat4;
use crate::tup::Tup;

pub fn translation<I: Into<f64>>(dx: I, dy: I, dz: I) -> Mat4 {
    let mut mat = Mat4::identity_matrix();
    const FINAL_COL: usize = Mat4::SIZE - 1;
    mat[(0, FINAL_COL)] = dx.into();
    mat[(1, FINAL_COL)] = dy.into();
    mat[(2, FINAL_COL)] = dz.into();
    mat
}

pub fn scaling<I: Into<f64>>(dx: I, dy: I, dz: I) -> Mat4 {
    let mut mat = Mat4::identity_matrix();
    mat[(0, 0)] = dx.into();
    mat[(1, 1)] = dy.into();
    mat[(2, 2)] = dz.into();
    mat
}

pub fn reflect_x() -> Mat4 {
    scaling(-1.0, 1.0, 1.0)
}

pub fn reflect_y() -> Mat4 {
    scaling(1.0, -1.0, 1.0)
}

pub fn reflect_z() -> Mat4 {
    scaling(1.0, 1.0, -1.0)
}

pub fn rotation_x(radians: f64) -> Mat4 {
    let mut mat = Mat4::identity_matrix();
    mat[(1, 1)] = radians.cos();
    mat[(1, 2)] = -radians.sin();
    mat[(2, 1)] = radians.sin();
    mat[(2, 2)] = radians.cos();
    mat
}

pub fn rotation_y(radians: f64) -> Mat4 {
    let mut mat = Mat4::identity_matrix();
    mat[(0, 0)] = radians.cos();
    mat[(0, 2)] = radians.sin();
    mat[(2, 0)] = -radians.sin();
    mat[(2, 2)] = radians.cos();
    mat
}

pub fn rotation_z(radians: f64) -> Mat4 {
    let mut mat = Mat4::identity_matrix();
    mat[(0, 0)] = radians.cos();
    mat[(0, 1)] = -radians.sin();
    mat[(1, 0)] = radians.sin();
    mat[(1, 1)] = radians.cos();
    mat
}

pub fn shearing(dx_y: f64, dx_z: f64, dy_x: f64, dy_z: f64, dz_x: f64, dz_y: f64) -> Mat4 {
    let mut mat = Mat4::identity_matrix();
    mat[(0, 1)] = dx_y;
    mat[(0, 2)] = dx_z;
    mat[(1, 0)] = dy_x;
    mat[(1, 2)] = dy_z;
    mat[(2, 0)] = dz_x;
    mat[(2, 1)] = dz_y;
    mat
}

pub fn view_transform(from: Tup, to: Tup, up: Tup) -> Mat4 {
    let forwardv = (to - from).normalize();
    let leftv = forwardv.cross(&up.normalize());
    let upv = leftv.cross(&forwardv);
    let orientation = Mat4::from_data(&[
        leftv.x,
        leftv.y,
        leftv.z,
        0.0,
        upv.x,
        upv.y,
        upv.z,
        0.0,
        -forwardv.x,
        -forwardv.y,
        -forwardv.z,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]);
    orientation * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tramsforms_test {
    use super::*;
    use crate::tup::Tup;
    use std::f64::consts;

    #[test]
    fn multiplying_by_translation_matrix_moves_point() {
        let transform: Mat4 = translation(5.0, -3.0, 2.0);
        let p = Tup::point(-3.0, 4.0, 5.0);
        let expected = Tup::point(2.0, 1.0, 7.0);
        assert_eq!(expected, transform * p)
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix_moves_point_in_reverse() {
        let transform: Mat4 = translation(5.0, -3.0, 2.0);
        let p = Tup::point(-3.0, 4.0, 5.0);
        let inverse = transform.inverse();
        let expected = Tup::point(-8.0, 7.0, 3.0);
        assert_eq!(expected, inverse * p)
    }

    #[test]
    fn multiplying_by_translation_matrix_does_not_affect_vector() {
        let transform: Mat4 = translation(5.0, -3.0, 2.0);
        let p = Tup::vector(-3.0, 4.0, 5.0);
        assert_eq!(p, transform * p)
    }

    #[test]
    fn multiplying_by_a_scaling_matrix_scales_all_values_in_point() {
        let transform: Mat4 = scaling(2.0, 3.0, 4.0);
        let p = Tup::point(-4.0, 6.0, 8.0);
        let expected = Tup::point(-8.0, 18.0, 32.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn multiplyinng_by_a_scaling_matrix_scales_the_length_of_a_vector() {
        let transform: Mat4 = scaling(2.0, 3.0, 4.0);
        let v = Tup::vector(-4.0, 6.0, 8.0);
        let expected = Tup::vector(-8.0, 18.0, 32.0);
        assert_eq!(expected, transform * v);
    }

    #[test]
    fn multiplying_by_inverse_scaling_matrix_scales_in_opposite_dir() {
        let transform: Mat4 = scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse();
        let v = Tup::vector(-4.0, 6.0, 8.0);
        let expected = Tup::vector(-2.0, 2.0, 2.0);
        assert_eq!(expected, inverse * v);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform: Mat4 = scaling(-1.0, 1.0, 1.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(-2.0, 3.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn multiplying_by_reflect_x_matrix_reflects_a_point_across_the_x_axis() {
        let transform: Mat4 = reflect_x();
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(-2.0, 3.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn multiplying_by_reflect_y_matrix_reflects_a_point_across_the_y_axis() {
        let transform: Mat4 = reflect_y();
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(2.0, -3.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn multiplying_by_reflect_z_matrix_reflects_a_point_across_the_z_axis() {
        let transform: Mat4 = reflect_z();
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(2.0, 3.0, -4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn mutriplying_by_rotation_x_can_rotate_a_point_around_the_x_axis_one_eighth() {
        let half_quarter: Mat4 = rotation_x(consts::PI / 4.0);
        let p = Tup::point(0.0, 1.0, 0.0);
        let expected = Tup::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        assert_eq!(expected, half_quarter * p);
    }

    #[test]
    fn mutriplying_by_rotation_x_can_rotate_a_point_around_the_x_axis_one_quarter() {
        let quarter: Mat4 = rotation_x(consts::PI / 2.0);
        let p = Tup::point(0.0, 1.0, 0.0);
        let expected = Tup::point(0.0, 0.0, 1.0);
        assert_eq!(expected, quarter * p);
    }

    #[test]
    fn inverse_of_rotation_x_rotates_in_opposite_direction() {
        let half_quarter: Mat4 = rotation_x(consts::PI / 4.0);
        let inverse = half_quarter.inverse();
        let p = Tup::point(0.0, 1.0, 0.0);
        let expected = Tup::point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);
        assert_eq!(expected, inverse * p);
    }

    #[test]
    fn mutriplying_by_rotation_y_can_rotate_a_point_around_the_y_axis_one_eighth() {
        let half_quarter: Mat4 = rotation_y(consts::PI / 4.0);
        let p = Tup::point(0.0, 0.0, 1.0);
        let expected = Tup::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        assert_eq!(expected, half_quarter * p);
    }

    #[test]
    fn mutriplying_by_rotation_y_can_rotate_a_point_around_the_y_axis_one_half() {
        let quarter: Mat4 = rotation_y(consts::PI / 2.0);
        let p = Tup::point(0.0, 0.0, 1.0);
        let expected = Tup::point(1.0, 0.0, 0.0);
        assert_eq!(expected, quarter * p);
    }

    #[test]
    fn mutriplying_by_rotation_z_can_rotate_a_point_around_the_z_axis_one_eighth() {
        let half_quarter: Mat4 = rotation_z(consts::PI / 4.0);
        let p = Tup::point(0.0, 1.0, 0.0);
        let expected = Tup::point(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        assert_eq!(expected, half_quarter * p);
    }

    #[test]
    fn mutriplying_by_rotation_z_can_rotate_a_point_around_the_z_axis_one_half() {
        let quarter: Mat4 = rotation_z(consts::PI / 2.0);
        let p = Tup::point(0.0, 1.0, 0.0);
        let expected = Tup::point(-1.0, 0.0, 0.0);
        assert_eq!(expected, quarter * p);
    }

    #[test]
    fn mutliplying_by_shearing_transfomation_moves_x_in_proportion_to_y() {
        let transform: Mat4 = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(5.0, 3.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn mutliplying_by_shearing_transfomation_moves_x_in_proportion_to_z() {
        let transform: Mat4 = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(6.0, 3.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn mutliplying_by_shearing_transfomation_moves_y_in_proportion_to_x() {
        let transform: Mat4 = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(2.0, 5.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn mutliplying_by_shearing_transfomation_moves_y_in_proportion_to_z() {
        let transform: Mat4 = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(2.0, 7.0, 4.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn mutliplying_by_shearing_transfomation_moves_z_in_proportion_to_x() {
        let transform: Mat4 = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(2.0, 3.0, 6.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn mutliplying_by_shearing_transfomation_moves_z_in_proportion_to_y() {
        let transform: Mat4 = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tup::point(2.0, 3.0, 4.0);
        let expected = Tup::point(2.0, 3.0, 7.0);
        assert_eq!(expected, transform * p);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tup::point(1.0, 0.0, 1.0);

        let rot = rotation_x(consts::PI / 2.0);
        let p2 = rot * p;
        assert_eq!(Tup::point(1.0, -1.0, 0.0), p2);

        let scale = scaling(5.0, 5.0, 5.0);
        let p3 = scale * p2;
        assert_eq!(Tup::point(5.0, -5.0, 0.0), p3);

        let trans = translation(10.0, 5.0, 7.0);
        let p4 = trans * p3;
        assert_eq!(Tup::point(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn chained_transformations_are_applied_in_reverse_order() {
        let p = Tup::point(1.0, 0.0, 1.0);
        let rot = rotation_x(consts::PI / 2.0);
        let scale = scaling(5.0, 5.0, 5.0);
        let trans = translation(10.0, 5.0, 7.0);

        let transform = trans * scale * rot;
        assert_eq!(Tup::point(15.0, 0.0, 7.0), transform * p);
    }

    #[test]
    fn default_view_transform_orientation_returns_identity_matrix() {
        let from = Tup::point(0, 0, 0);
        let to = Tup::point(0, 0, -1);
        let up = Tup::vector(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(Mat4::identity_matrix(), t);
    }

    #[test]
    fn view_transform_looking_in_positive_z_direction() {
        let from = Tup::point(0, 0, 0);
        let to = Tup::point(0, 0, 1);
        let up = Tup::vector(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(reflect_z() * reflect_x(), t);
    }

    #[test]
    fn view_transform_moves_the_world() {
        let from = Tup::point(0, 0, 8);
        let to = Tup::point(0, 0, 0);
        let up = Tup::vector(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(translation(0, 0, -8), t);
    }

    #[test]
    fn view_transform_can_accur_in_arbitrary_directions() {
        let from = Tup::point(1, 3, 2);
        let to = Tup::point(4, -2, 8);
        let up = Tup::vector(1, 1, 0);
        let t = view_transform(from, to, up);
        let expected = Mat4::from_data(&[
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
            0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
        ]);
        assert_eq!(expected, t);
    }
}
