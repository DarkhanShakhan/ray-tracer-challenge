use super::{
    intersections::Intersection, materials::Material, matrice::Matrice, rays::Ray, tuple::Tuple,
};

pub trait Shape {
    fn set_transform(&mut self, transformation: Matrice);
    fn get_transform(&self) -> Matrice;
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
}

#[cfg(test)]
mod shape_tests {
    use std::f32::consts::PI;

    use crate::features::{
        materials::Material,
        matrice::Matrice,
        spheres::Sphere,
        transformations::{rotation_z, scaling, translation},
        tuple::Tuple,
    };

    use super::Shape;

    fn test_shape() -> impl Shape {
        Sphere::new()
    }

    #[test]
    fn testing_default_transform() {
        let shape = test_shape();
        assert_eq!(shape.get_transform(), Matrice::identity_matrix(4))
    }

    #[test]
    fn testing_assigning_transform() {
        let mut shape = test_shape();
        shape.set_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(shape.get_transform(), translation(2.0, 3.0, 4.0))
    }

    #[test]
    fn testing_default_material() {
        let shape = test_shape();
        let sphere = Sphere::new();
        assert_eq!(shape.get_material(), sphere.material)
    }

    #[test]
    fn testing_assigning_material() {
        let mut shape = test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        shape.set_material(m.clone());
        assert_eq!(shape.get_material(), m)
    }

    #[test]
    fn testing_normal_on_translated_shape() {
        let mut shape = test_shape();
        shape.set_transform(translation(0.0, 1.0, 0.0));
        let n = shape.normal_at(Tuple::point(0.0, 1.70711, -std::f32::consts::FRAC_1_SQRT_2));
        assert_eq!(
            n,
            Tuple::vector(
                0.0,
                std::f32::consts::FRAC_1_SQRT_2,
                -std::f32::consts::FRAC_1_SQRT_2
            )
        )
    }
    #[test]
    fn testing_normal_on_transformed_shape() {
        let mut shape = test_shape();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        shape.set_transform(m);
        let n = shape.normal_at(Tuple::point(
            0.0,
            2.0_f32.sqrt() / 2.0,
            -(2.0_f32.sqrt() / 2.0),
        ));
        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254))
    }
}
