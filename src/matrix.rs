use crate::math_helpers::nearly_eq;
use crate::tup::Tup;
use std::ops::{Index, IndexMut, Mul} ;

#[derive(Debug, Clone)]
pub struct Mat4 {
   data: Vec<f64>,
}

impl Mat4 {
    const SIZE: usize = 4;
   
    pub fn from_data( data: &[f64]) -> Self {
        assert!(data.len() == Self::SIZE * Self::SIZE);
        Self {
            data: data.iter().cloned().collect(),
        }
    }
    
    pub fn new() -> Self {
        Mat4::from_data(&vec![0.0; Self::SIZE * Self::SIZE])
    }

    pub fn identity_matrix() -> Self {
        Self::from_data(&vec![
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = Mat4::new();
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                transposed[(row, col)] = self[(col, row)];
            }
        }
        transposed
    }

    fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Mat3 {
        let mut sub = Vec::new();
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                if row != row_to_remove && col != col_to_remove {
                    sub.push(self[(row, col)]);
                }
            }
        }
        Mat3::from_data(&sub)
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col)
            .determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let factor = if (row + col) % 2 == 0 { 1 } else { -1 };
        factor as f64 * self.minor(row, col)
    }

    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        let row = 0;
        for col in 0..Self::SIZE {
            determinant += self[(row, col)] * self.cofactor(row, col);
        }
        determinant
    }

    pub fn is_invertable(&self) -> bool {
        !nearly_eq(0.0, self.determinant())
    }

    pub fn inverse(&self) -> Self {
        assert!(self.is_invertable());
        let mut inverse = Self::new();
        let determinant = self.determinant();
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                let c = self.cofactor(row, col);
                inverse[(col, row)] = c / determinant;
            }
        }
        inverse
    }
}

impl Index<(usize, usize)> for Mat4 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * Self::SIZE + col]
    }
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * Self::SIZE + col]
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter()
            .zip(other.data.iter())
            .all(|(&a, &b)| nearly_eq(a, b))
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut m = Mat4::new();
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                m[(row, col)] =
                    self[(row, 0)]  * other[(0, col)]
                    + self[(row, 1)]  * other[(1, col)]
                    + self[(row, 2)]  * other[(2, col)]
                    + self[(row, 3)]  * other[(3, col)]
            }
        }
        m
    }
}

impl Mul<Tup> for Mat4 {
    type Output = Tup;

    fn mul(self, other: Tup) -> Self::Output {
        let mut t_data = Vec::new();
        for row in 0..Self::SIZE {
            let coord =
                self[(row, 0)] * other.x
                + self[(row, 1)] * other.y
                + self[(row, 2)] * other.z
                + self[(row, 3)] * other.w;
            t_data.push(coord);
        }
        Tup::new(t_data[0], t_data[1], t_data[2], t_data[3])
    }
}

#[derive(Debug, Clone)]
struct Mat3 {
    data: Vec<f64>,
}

impl Mat3 {
    const SIZE: usize = 3;

    fn from_data(data: &[f64]) -> Self {
        assert!(data.len() == Self::SIZE * Self::SIZE);
        Self {
            data: data.iter().cloned().collect(),
        }
    }
    
    fn new(width: usize, height: usize) -> Self {
        Self::from_data (&vec![0.0; width * height])
    }

    fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Mat2 {
        let mut sub = Vec::new();
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                if row != row_to_remove && col != col_to_remove {
                    sub.push(self[(row, col)]);
                }
            }
        }
        Mat2::from_data(&sub)
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col)
            .determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let factor = if (row + col) % 2 == 0 { 1 } else { -1 };
        factor as f64 * self.minor(row, col)
    }

    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        let row = 0;
        for col in 0..Self::SIZE {
            determinant += self[(row, col)] * self.cofactor(row, col);
        }
        determinant
    }
}

impl Index<(usize, usize)> for Mat3 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * Self::SIZE + col]
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter()
            .zip(other.data.iter())
            .all(|(&a, &b)| nearly_eq(a, b))
    }
}


#[derive(Debug, Clone)]
struct Mat2 {
    data: Vec<f64>,
}

impl Mat2 {
    const SIZE: usize = 2;

    fn from_data(data: &[f64]) -> Self {
        assert!(data.len() == Self::SIZE * Self::SIZE);
        Self {
            data: data.iter().cloned().collect(),
        }
    }
    
    fn new(width: usize, height: usize) -> Self {
        Self::from_data (&vec![0.0; width * height])
    }

    fn determinant(&self) -> f64 {
        // | a, b |
        // | c, d |
        // ad - bc == determinant
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }

}

impl Index<(usize, usize)> for Mat2 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * Self::SIZE + col]
    }
}


impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter()
            .zip(other.data.iter())
            .all(|(&a, &b)| nearly_eq(a, b))
    }
}

#[cfg(test)]
mod matrix_tests  {
    use super::*;

    fn assert_nearly_eq(a: f64, b: f64) {
        assert!(nearly_eq(a, b));
    }

    #[test]
    fn construct_and_inspect_a_4x4_matrix() {
        let data = vec![
            1.0,  2.0,  3.0,  4.0,
            5.5,  6.5,  7.5,  8.5,
            9.0,  10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ];
        let m = Mat4::from_data(&data);
        assert!(nearly_eq(11.0, m[(2, 2)]));
    }

    #[test]
    fn construct_and_inspect_a_2x2_matrix() {
        let data = vec![-3.0, -5.0, 1.0, -2.0];
        let m = Mat2::from_data(&data);
        assert!(nearly_eq(-2.0, m[(1, 1)]));
    }

    #[test]
    fn construct_and_inspect_a_3x3_matrix() {
        let data = vec![
            -3.0, -5.0, 0.0,
             1.0, -2.0, 7.0,
             0.0,  1.0, 1.0
        ];
        let m = Mat3::from_data(&data);
        assert!(nearly_eq(-2.0, m[(1, 1)]));
    }

    #[test]
    fn identical_matrices_are_equal() {
        let m1 = Mat4::from_data(&vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]); 
        let m2 = Mat4::from_data(&vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);
        assert!(m1 == m2);
    }

    #[test]
    fn different_matrices_are_not_equal() {
        let m1 = Mat4::from_data(&vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]); 
        let m2 = Mat4::from_data(&vec![
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        ]);
        assert!(m1 != m2);
    }

    #[test]
    fn matrices_can_be_multiplied_by_other_matrices() {
        let m1 = Mat4::from_data(&vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]); 
        let m2 = Mat4::from_data(&vec![
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0
        ]);

        let expected = Mat4::from_data(&vec![
            20.0, 22.0, 50.0,  48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0,  42.0
        ]);
        assert_eq!(expected, m1 * m2);
    }

    #[test]
    fn matrices_can_be_multiplied_by_tuples() {
        let m =  Mat4::from_data(&vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        ]);
        let t = Tup::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tup::new(18.0, 24.0, 33.0, 1.0);
        assert_eq!(expected, m * t);
    }

    #[test]
    fn multiplying_a_matrix_by_identity_matrix_yields_original() {
        let m =  Mat4::from_data(&vec![
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        ]);
        let result = m.clone() * Mat4::identity_matrix();
        assert_eq!(m, result);
    }

    #[test]
    fn a_matrix_can_be_transposed() {
        let m = Mat4::from_data(&vec![
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.8,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        ]);
        let t = Mat4::from_data(&vec![
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.8, 3.0, 8.0
        ]);
        assert_eq!(t, m.transpose());
    }

    #[test]
    fn the_transpose_of_the_identity_matrix_is_the_identity_matrix() {
        assert_eq!(Mat4::identity_matrix(), Mat4::identity_matrix().transpose());
    }

    #[test]
    fn the_determinant_of_a_2x2_matrix_can_be_calculated() {
        let m = Mat2::from_data(&vec![
             1.0, 5.0, 
            -3.0, 2.0,
        ]);
        assert_nearly_eq(17.0, m.determinant())
    }

    #[test]
    fn the_submatrix_of_a_mat4_is_a_mat3() {
        let m = Mat4::from_data(&vec![
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        ]);
        let row_to_remove = 2;
        let col_to_remove = 1;
        let expected = Mat3::from_data(&vec![
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0
        ]);
        assert_eq!(expected, m.submatrix(row_to_remove, col_to_remove));
    }

    #[test]
    fn the_submatrix_of_a_mat3_is_a_mat2() {
        let m = Mat3::from_data(&vec![
             1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
             0.0, 6.0, -3.0
        ]);
        let row_to_remove = 0;
        let col_to_remove = 2;
        let expected = Mat2::from_data(&vec![
            -3.0, 2.0,
             0.0, 6.0
        ]);
        assert_eq!(expected, m.submatrix(row_to_remove, col_to_remove));
    }

    #[test]
    fn the_minor_of_an_element_of_a_mat3_can_be_calculated() {
        let m = Mat3::from_data(&vec![
             3.0,  5.0, 0.0,
             2.0, -1.0, -7.0,
             6.0, -1.0, 5.0
        ]);
        let row = 1;
        let col = 0;
        assert_nearly_eq(25.0, m.minor(row, col));
    }

    #[test]
    fn the_cofactor_of_element_0_0_of_a_mat3_does_not_change_signs() {
        let m = Mat3::from_data(&vec![
             3.0,  5.0, 0.0,
             2.0, -1.0, -7.0,
             6.0, -1.0, 5.0
        ]);
        let row = 0;
        let col = 0;
        assert_nearly_eq(m.minor(row, col), m.cofactor(row, col));
    }

    #[test]
    fn the_cofactor_of_element_1_0_of_a_mat3_does_change_signs() {
        let m = Mat3::from_data(&vec![
             3.0,  5.0, 0.0,
             2.0, -1.0, -7.0,
             6.0, -1.0, 5.0
        ]);
        let row = 1;
        let col = 0;
        assert_nearly_eq(-m.minor(row, col), m.cofactor(row, col));
    }

    #[test]
    fn the_determinant_of_a_mat3_can_be_calculated() {
        let m = Mat3::from_data(&vec![
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        ]);
        assert_nearly_eq(56.0, m.cofactor(0, 0));
        assert_nearly_eq(12.0, m.cofactor(0, 1));
        assert_nearly_eq(-46.0, m.cofactor(0, 2));
        assert_nearly_eq(-196.0, m.determinant());
    }

    #[test]
    fn the_determinant_of_a_mat4_can_be_calculated() {
        let m = Mat4::from_data(&vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ]);
        assert_nearly_eq(690.0, m.cofactor(0, 0));
        assert_nearly_eq(447.0, m.cofactor(0, 1));
        assert_nearly_eq(210.0, m.cofactor(0, 2));
        assert_nearly_eq(51.0, m.cofactor(0, 3));
        assert_nearly_eq(-4071.0, m.determinant());
    }

    #[test]
    fn a_matrix_with_a_nonzero_determinant_is_invertable() {
        let m = Mat4::from_data(&vec![
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0
        ]);
        assert_nearly_eq(-2120.0, m.determinant());
        assert!(m.is_invertable());
    }

    #[test]
    fn a_matrix_with_a_zero_determinant_is_not_invertable() {
        let m = Mat4::from_data(&vec![
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0
        ]);
        assert_nearly_eq(0.0, m.determinant());
        assert!(!m.is_invertable());
    }

    #[test]
    fn the_inverse_of_an_invertable_matrix_can_be_calculated() {
        let m = Mat4::from_data(&vec![
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0
        ]);
        let expected = Mat4::from_data(&vec![
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639
        ]);
        assert_eq!(expected, m.inverse());
    }

    #[test]
    fn the_inverse_of_a_second_matrix_can_be_calculated() {
        let m = Mat4::from_data(&vec![
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0
        ]);
        let expected = Mat4::from_data(&vec![
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897,  0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308
        ]);
        assert_eq!(expected, m.inverse());        
    }

    #[test]
    fn the_inverse_of_a_third_matrix_can_be_calculated() {
        let m = Mat4::from_data(&vec![
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0
        ]);
        let expected = Mat4::from_data(&vec![
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333
        ]);
        assert_eq!(expected, m.inverse());        
    }

    #[test]
    fn multiplying_a_product_by_its_inverse_yields_original_matrix() {
        let m_a = Mat4::from_data(&vec![
            3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0
        ]);
        let m_b = Mat4::from_data(&vec![
            8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0
        ]);
        let product = m_a.clone() * m_b.clone();
        assert_eq!(m_a, product * m_b.inverse());
    }
}
