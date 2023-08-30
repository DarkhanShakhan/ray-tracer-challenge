use rand::prelude::*;
use std::ops::Mul;

use super::tuple::{Tuple, TupleType};

#[derive(PartialEq, Debug, Clone)]
pub struct Matrice {
    pub size: usize,
    pub matrice: Vec<Vec<f32>>,
}

impl Eq for Matrice {}

impl Matrice {
    //TODO: add 2d matrix as input
    pub fn new(size: usize) -> Self {
        Matrice {
            size,
            matrice: vec![vec![0.0; size]; size],
        }
    }

    fn random(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut out = Self::new(size);
        for ix in 0..size {
            for jx in 0..size {
                out.write_element(ix, jx, rng.gen())
            }
        }
        out
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        let mut out = Self::new(self.size);
        let mut c;
        for row in 0..self.size {
            for col in 0..self.size {
                c = self.cofactor(row, col);
                out.write_element(col, row, c / det);
            }
        }
        Some(out)
    }
    pub fn determinant(&self) -> f32 {
        if self.size == 2 {
            return self.matrice[0][0] * self.matrice[1][1]
                - self.matrice[0][1] * self.matrice[1][0];
        }
        let mut det = 0.0;
        for c in 0..self.size {
            det += self.matrice[0][c] * self.cofactor(0, c);
        }
        det
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Self {
        let mut matrice = vec![];
        let mut row_to_add = vec![];
        for row in 0..self.size {
            if row == r {
                continue;
            }
            for column in 0..self.size {
                if column == c {
                    continue;
                }
                row_to_add.push(self.element_at(row, column));
            }
            matrice.push(row_to_add);
            row_to_add = vec![];
        }
        Self {
            size: self.size - 1,
            matrice,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn element_at(&self, row: usize, column: usize) -> f32 {
        self.matrice[row][column]
    }
    pub fn write_element(&mut self, row: usize, column: usize, element: f32) {
        self.matrice[row][column] = element;
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut out = Matrice {
            size,
            matrice: vec![vec![0.0; size]; size],
        };
        for ix in 0..size {
            out.write_element(ix, ix, 1.0);
        }
        out
    }
    pub fn transpose(&self) -> Self {
        let mut out = Self::new(self.size());
        for ix in 0..self.size {
            for jx in 0..self.size {
                out.write_element(ix, jx, self.element_at(jx, ix));
            }
        }
        out
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        if (row + column) % 2 != 0 {
            return -self.minor(row, column);
        }
        self.minor(row, column)
    }
}

impl Mul for Matrice {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::new(self.size);
        let mut element = 0.0;
        for ix in 0..self.size {
            for jx in 0..self.size {
                for kx in 0..self.size {
                    element += self.element_at(ix, kx) * rhs.element_at(kx, jx);
                }
                out.write_element(ix, jx, element);
                element = 0.0;
            }
        }
        out
    }
}

impl Mul<Tuple> for Matrice {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut tuple = vec![rhs.x, rhs.y, rhs.z, 0.0];
        if rhs.w == TupleType::Point {
            tuple[3] = 1.0;
        }
        let mut out = vec![0.0; 4];
        for (ix, row) in self.matrice.iter().enumerate() {
            for (jx, col) in row.iter().enumerate() {
                out[ix] += *col * tuple[jx];
            }
        }
        Tuple::new(out[0], out[1], out[2], rhs.w)
    }
}

#[cfg(test)]
mod matrice_tests {
    use std::vec;

    use crate::features::tuple::Tuple;

    use super::Matrice;

    #[test]
    fn test_matrice_equal() {
        let mut matrice = Matrice::new(4);
        matrice.write_element(0, 2, 4.5);
        let mut matrice_other = Matrice::new(4);
        matrice_other.write_element(0, 2, 4.5);
        assert_eq!(matrice, matrice_other);
    }

    #[test]
    fn test_matrice_not_equal() {
        let mut matrice = Matrice::new(4);
        matrice.write_element(0, 2, 6.5);
        let mut matrice_other = Matrice::new(4);
        matrice_other.write_element(0, 2, 4.5);
        assert!(matrice != matrice_other);
    }

    #[test]
    fn test_matrice_mul() {
        let matrice = Matrice::random(4);
        let other_matrice = Matrice::random(4);
        println!("{:?}", matrice * other_matrice);
    }

    #[test]
    fn test_identity_matrice() {
        let identity_matrix = Matrice::identity_matrix(4);
        let mut matrice = Matrice::new(4);
        matrice.write_element(1, 2, 0.7);
        matrice.write_element(2, 1, 7.5);
        assert_eq!(matrice, matrice.clone() * identity_matrix);
    }

    #[test]
    fn test_transpose_matrice() {
        let matrice = Matrice::random(4);
        let transposed_matrice = matrice.transpose();
        assert_eq!(matrice, transposed_matrice.transpose());
    }

    #[test]
    fn test_determinant() {
        let mut matrice = Matrice::random(2);
        matrice.write_element(0, 0, 1.0);
        matrice.write_element(0, 1, 5.0);
        matrice.write_element(1, 0, -3.0);
        matrice.write_element(1, 1, 2.0);
        assert_eq!(matrice.determinant(), 17.0);
    }

    #[test]
    fn test_submatrice() {
        let matrice = Matrice::random(4);
        let sub_matrice = matrice.submatrix(2, 2);
        println!("{:?}", sub_matrice);
    }

    #[test]
    fn test_minor() {
        let mut matrice = Matrice::new(3);
        matrice.write_element(0, 0, 3.0);
        matrice.write_element(0, 1, 5.0);
        matrice.write_element(0, 2, 0.0);
        matrice.write_element(1, 0, 2.0);
        matrice.write_element(1, 1, -1.0);
        matrice.write_element(1, 2, -7.0);
        matrice.write_element(2, 0, 6.0);
        matrice.write_element(2, 1, -1.0);
        matrice.write_element(2, 2, 5.0);
        assert_eq!(matrice.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let mut matrice = Matrice::new(3);
        matrice.write_element(0, 0, 3.0);
        matrice.write_element(0, 1, 5.0);
        matrice.write_element(0, 2, 0.0);
        matrice.write_element(1, 0, 2.0);
        matrice.write_element(1, 1, -1.0);
        matrice.write_element(1, 2, -7.0);
        matrice.write_element(2, 0, 6.0);
        matrice.write_element(2, 1, -1.0);
        matrice.write_element(2, 2, 5.0);
        assert_eq!(matrice.cofactor(1, 0), -25.0);
        assert_eq!(matrice.cofactor(0, 0), -12.0);
    }

    #[test]
    fn test_3d_determinant() {
        let matrice = Matrice {
            size: 3,
            matrice: vec![
                vec![1.0, 2.0, 6.0],
                vec![-5.0, 8.0, -4.0],
                vec![2.0, 6.0, 4.0],
            ],
        };
        assert_eq!(matrice.determinant(), -196.0);
    }

    #[test]
    fn test_4d_determinant() {
        let matrice = Matrice {
            size: 4,
            matrice: vec![
                vec![-2.0, -8.0, 3.0, 5.0],
                vec![-3.0, 1.0, 7.0, 3.0],
                vec![1.0, 2.0, -9.0, 6.0],
                vec![-6.0, 7.0, 7.0, -9.0],
            ],
        };
        assert_eq!(matrice.determinant(), -4071.0);
    }

    #[test]
    fn test_is_invertible() {
        let matrice = Matrice {
            size: 4,
            matrice: vec![
                vec![6.0, 4.0, 4.0, 4.0],
                vec![5.0, 5.0, 7.0, 6.0],
                vec![4.0, -9.0, 3.0, -7.0],
                vec![9.0, 1.0, 7.0, -6.0],
            ],
        };
        let det = matrice.determinant();
        assert!(det != 0.0);
    }
    #[test]
    fn test_is_not_invertible() {
        let matrice = Matrice {
            size: 4,
            matrice: vec![
                vec![-4.0, 2.0, -2.0, -3.0],
                vec![9.0, 6.0, 2.0, 6.0],
                vec![0.0, -5.0, 1.0, -5.0],
                vec![0.0, 0.0, 0.0, 0.0],
            ],
        };
        let det = matrice.determinant();
        assert!(det == 0.0);
    }

    #[test]
    fn test_inverse_matrix() {
        let matrice = Matrice {
            size: 4,
            matrice: vec![
                vec![8.0, -5.0, 9.0, 2.0],
                vec![7.0, 5.0, 6.0, 1.0],
                vec![-6.0, 0.0, 9.0, 6.0],
                vec![-3.0, 0.0, -9.0, -4.0],
            ],
        };
        println!("{:?}", matrice.inverse());
    }

    #[test]
    fn test_multiplying_inverse() {
        let matrice_a = Matrice::random(4);
        let matrice_b = Matrice::random(4);
        let matrice_c = matrice_a.clone() * matrice_b.clone();
        println!("{:?}", matrice_c * matrice_b.inverse().unwrap());
        println!("{:?}", matrice_a);
    }

    #[test]
    fn test_inverse_identity_matrice() {
        let id_matrice = Matrice::identity_matrix(4);
        let id_matrice_inverse = id_matrice.inverse().unwrap();
        println!("{:?}", id_matrice);
        println!("{:?}", id_matrice_inverse);
    }

    #[test]
    fn test_multiplying_inverse_to_matrice() {
        let matrice = Matrice {
            size: 4,
            matrice: vec![
                vec![8.0, -5.0, 9.0, 2.0],
                vec![7.0, 5.0, 6.0, 1.0],
                vec![-6.0, 0.0, 9.0, 6.0],
                vec![-3.0, 0.0, -9.0, -4.0],
            ],
        };
        let inverse = matrice.inverse().unwrap();
        println!("{:?}", matrice * inverse);
    }

    #[test]
    fn test_multiplying_matrix_to_tuple() {
        let matrice = Matrice {
            size: 4,
            matrice: vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![2.0, 4.0, 4.0, 2.0],
                vec![8.0, 6.0, 4.0, 1.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        };
        let tuple = Tuple::point(1.0, 2.0, 3.0);
        let res = matrice * tuple;
        assert_eq!(res, Tuple::point(18.0, 24.00, 33.0))
    }
}
