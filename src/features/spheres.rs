use uuid::Uuid;

use super::{
    matrice::Matrice,
    rays::Ray,
    tuple::{Tuple, TupleType},
};

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrice,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrice::identity_matrix(4),
        }
    }
    pub fn set_transform(&mut self, t: Matrice) {
        self.transform = t;
    }
    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = TupleType::Vector;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod sphere_tests {
    use crate::features::transformations::translation;

    use super::Sphere;

    #[test]
    fn test_set_tranform() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t.clone());
        assert_eq!(s.transform, t);
    }
}

#[cfg(test)]
mod normals_tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    use crate::features::{
        transformations::{rotation_z, scaling, translation},
        tuple::Tuple,
    };

    use super::Sphere;

    #[test]
    fn test_normal_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        println!("{:?}", Tuple::point(1.0, 0.0, 0.0).normalize());
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn test_normal_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0))
    }

    #[test]
    fn test_normal_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0))
    }

    #[test]
    fn test_normal_on_nonaxial() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Tuple::vector(
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
            )
        )
    }
    #[test]
    fn test_normal_is_normalized() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_normal_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert_eq!(n, Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn test_normal_transformed_sphere() {
        let mut s = Sphere::new();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(Tuple::point(
            0.0,
            2.0_f32.sqrt() / 2.0,
            -(2.0_f32.sqrt() / 2.0),
        ));
        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }
}
