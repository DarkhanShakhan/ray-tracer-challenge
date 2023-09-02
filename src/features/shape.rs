use super::{
    intersections::Intersection, materials::Material, matrice::Matrice, rays::Ray, spheres::Sphere,
    tuple::Tuple,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        match self {
            Shape::Sphere(s) => s.intersect(r),
        }
    }
    pub fn set_transform(&mut self, t: Matrice) {
        match self {
            Shape::Sphere(s) => s.set_transform(t),
        }
    }
    pub fn transform(&self) -> Matrice {
        match self {
            Shape::Sphere(s) => s.transform.clone(),
        }
    }
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shape::Sphere(s) => s.normal_at(point),
        }
    }
    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material.clone(),
        }
    }
    pub fn set_material(&mut self, m: Material) {
        match self {
            Shape::Sphere(s) => s.material = m,
        }
    }
}

#[cfg(test)]
mod shape_tests {
    use crate::features::transformations::{scaling, translation};

    use super::*;

    fn test_shape() -> Shape {
        Shape::Sphere(Sphere::new())
    }

    #[test]
    fn default_transformation() {
        let s = test_shape();
        assert_eq!(s.transform(), Matrice::identity_matrix(4))
    }

    #[test]
    fn assigning_transformation() {
        let mut s = test_shape();
        s.set_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(s.transform(), translation(2.0, 3.0, 4.0))
    }

    #[test]
    fn default_material() {
        let s = test_shape();
        let m = s.material();
        assert_eq!(m, Material::new())
    }

    #[test]
    fn assigning_material() {
        let mut s = test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m.clone());
        assert_eq!(s.material(), m)
    }
    #[test]
    fn computing_normal_on_translated_shape() {
        let mut s = test_shape();
        s.set_transform(scaling(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -std::f32::consts::FRAC_1_SQRT_2));
        assert_eq!(
            n,
            Tuple::vector(
                0.0,
                std::f32::consts::FRAC_1_SQRT_2,
                -std::f32::consts::FRAC_1_SQRT_2
            )
        )
    }
}
