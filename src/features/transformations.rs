use super::matrice::Matrice;

pub fn translation(x: f32, y: f32, z: f32) -> Matrice {
    let mut out = Matrice::identity_matrix(4);
    out.write_element(0, 3, x);
    out.write_element(1, 3, y);
    out.write_element(2, 3, z);
    out
}
pub fn scaling(x: f32, y: f32, z: f32) -> Matrice {
    let mut out = Matrice::identity_matrix(4);
    out.write_element(0, 0, x);
    out.write_element(1, 1, y);
    out.write_element(2, 2, z);
    out
}

#[cfg(test)]
mod translation_tests {
    use crate::features::{transformations::translation, tuple::Tuple};

    #[test]
    fn test_multiplying_translation_matrix_to_point() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0))
    }

    #[test]
    fn test_multiplying_inverse_translation_matrix_to_point() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_multiplying_translation_matrix_to_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }
}

#[cfg(test)]
mod scaling_tests {
    use crate::features::{transformations::scaling, tuple::Tuple};
    #[test]
    fn test_scaling_matrix_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0))
    }

    #[test]
    fn test_scaling_matrix_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::vector(-8.0, 18.0, 32.0))
    }

    #[test]
    fn test_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(inv * p, Tuple::point(-2.0, 2.0, 2.0))
    }

    #[test]
    fn test_reflection_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0))
    }
}
