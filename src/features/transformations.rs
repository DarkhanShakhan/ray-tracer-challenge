use super::{matrice::Matrice, tuple::Tuple};

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

pub fn rotation_x(rad: f32) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(1, 1, rad.cos());
    matrice.write_element(1, 2, -rad.sin());
    matrice.write_element(2, 1, rad.sin());
    matrice.write_element(2, 2, rad.cos());
    matrice
}

pub fn rotation_y(rad: f32) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 0, rad.cos());
    matrice.write_element(0, 2, rad.sin());
    matrice.write_element(2, 0, -rad.sin());
    matrice.write_element(2, 2, rad.cos());
    matrice
}

pub fn rotation_z(rad: f32) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 0, rad.cos());
    matrice.write_element(0, 1, -rad.sin());
    matrice.write_element(1, 0, rad.sin());
    matrice.write_element(1, 1, rad.cos());
    matrice
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 1, xy);
    matrice.write_element(0, 2, xz);
    matrice.write_element(1, 0, yx);
    matrice.write_element(1, 2, yz);
    matrice.write_element(2, 0, zx);
    matrice.write_element(2, 1, zy);
    matrice
}

pub fn view_transformation(from: Tuple, to: Tuple, up: Tuple) -> Matrice {
    let forward = (to - from).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    Matrice {
        size: 4,
        matrice: vec![
            vec![left.x, left.y, left.z, 0.0],
            vec![true_up.x, true_up.y, true_up.z, 0.0],
            vec![-forward.x, -forward.y, -forward.z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ],
    } * translation(-from.x, -from.y, -from.z)
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
    use super::*;
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

#[cfg(test)]
mod rotation_tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_rotation_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_inverse_rotation_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();
        assert_eq!(
            inv * p,
            Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt() / 2.0))
        );
    }
    #[test]
    fn test_rotation_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::point(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::point(-(2.0_f32.sqrt() / 2.0), 2.0_f32.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }
}

#[cfg(test)]
mod shearing_tests {
    use super::*;

    #[test]
    fn test_shearing_transformation_x_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }
    #[test]
    fn test_shearing_transformation_y_to_z() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }
}

#[cfg(test)]
mod chained_tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_chained_tranformation() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}

#[cfg(test)]
mod view_transformation_tests {
    use super::*;

    #[test]
    fn test_default_view() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        assert_eq!(t, Matrice::identity_matrix(4))
    }

    #[test]
    fn test_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0))
    }

    #[test]
    fn test_moving_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        assert_eq!(t, translation(0.0, 0.0, -8.0))
    }

    #[test]
    fn test_arbitrary_view() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        println!("{:?}", t);
    }
}
