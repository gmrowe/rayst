use crate::math_helpers::nearly_eq;
use std::ops::{Index, IndexMut, Mul} ;


#[derive(Debug)]
pub struct Mat4 {
   data: Vec<f64>,
}

impl Mat4 {
    const WIDTH: usize = 4;
   
    fn from_data( data: &[f64]) -> Self {
        assert!(data.len() == Self::WIDTH * Self::WIDTH);
        Self {
            data: data.iter().cloned().collect(),
        }
    }
    
    fn new() -> Self {
        Mat4::from_data(&vec![0.0; Self::WIDTH * Self::WIDTH])
    }

}

impl Index<(usize, usize)> for Mat4 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * Self::WIDTH + col]
    }
}


impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * Self::WIDTH + col]
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
        for row in 0..Self::WIDTH {
            for col in 0..Self::WIDTH {
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

struct Mat2 {
    data: Vec<f64>,
}

impl Mat2 {
    const WIDTH: usize = 2;

    fn from_data(data: &[f64]) -> Self {
        assert!(data.len() == Self::WIDTH * Self::WIDTH);
        Self {
            data: data.iter().cloned().collect(),
        }
    }
    
    fn new(width: usize, height: usize) -> Self {
        Self::from_data (&vec![0.0; width * height])
    }

}

impl Index<(usize, usize)> for Mat2 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * Self::WIDTH + col]
    }
}

struct Mat3 {
    data: Vec<f64>,
}

impl Mat3 {
    const WIDTH: usize = 3;

    fn from_data(data: &[f64]) -> Self {
        assert!(data.len() == Self::WIDTH * Self::WIDTH);
        Self {
            data: data.iter().cloned().collect(),
        }
    }
    
    fn new(width: usize, height: usize) -> Self {
        Self::from_data (&vec![0.0; width * height])
    }

}

impl Index<(usize, usize)> for Mat3 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * Self::WIDTH + col]
    }
}

#[cfg(test)]
mod matrix_tests  {
    use super::*;
    
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
    fn identical_matrixes_are_equal() {
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
    fn different_matrixes_are_not_equal() {
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
    fn matrices_can_be_multiplied() {
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
}
