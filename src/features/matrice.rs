use std::ops::Mul;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrice {
    size:usize,
    matrice: Vec<Vec<f32>>
}

impl Matrice {
    pub fn new(size:usize) -> Self {
        Matrice{
            size:size,
            matrice:vec![vec![0.0;size];size]
        }
    }   
    pub fn element_at(&self,row:usize, column:usize) -> f32 {
        self.matrice[row][column]
    }
    pub fn write_element(&mut self,row:usize, column:usize, element:f32) {
        self.matrice[row][column] = element;
    }
}

impl Mul for Matrice {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut out= Self::new(self.size);
        let mut element = 0.0;
        for ix in 0..self.size {
            for jx in 0..self.size {
                for kx in 0..self.size{
                    element += self.element_at(ix, kx)*rhs.element_at(kx, jx);
                }
                out.write_element(ix,jx, element);
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
        let mut matrice = Matrice::new(2);
        matrice.write_element(0, 0, 1.0);
        matrice.write_element(0, 1, 2.0);
        matrice.write_element(1, 0, 3.0);
        matrice.write_element(1, 1, 4.0);
        let mut other_matrice = Matrice::new(2);
        other_matrice.write_element(0, 0, 1.0);
        other_matrice.write_element(0, 1, 2.0);
        other_matrice.write_element(1, 0, 3.0);
        other_matrice.write_element(1, 1, 4.0);
        println!("{:?}", matrice * other_matrice);
    }
}