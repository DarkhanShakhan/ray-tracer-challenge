use rand::prelude::*;
use std::ops::Mul;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrice {
    size: usize,
    matrice: Vec<Vec<f32>>,
}

impl Matrice {
    pub fn new(size: usize) -> Self {
        Matrice {
            size,
            matrice: vec![vec![0.0; size]; size],
        }
    }

    pub fn random(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut out = Self::new(size);
        for ix in 0..size {
            for jx in 0..size {
                out.write_element(ix, jx, rng.gen())
            }
        }
        out
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

#[cfg(test)]
mod matrice_tests {
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
}
