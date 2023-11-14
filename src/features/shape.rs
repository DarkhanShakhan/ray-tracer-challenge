use super::{
    cube::Cube,
    intersections::Intersection,
    materials::Material,
    matrice::Matrice,
    planes::Plane,
    rays::{transform, Ray},
    spheres::Sphere,
    tuple::Tuple,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

impl Shape {
    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let ray = transform(r, self.transform().inverse().unwrap());
        match self {
            Shape::Sphere(s) => s.intersect(&ray),
            Shape::Plane(p) => p.intersect(&ray),
            Shape::Cube(c) => c.intersect(&ray),
        }
    }
    pub fn set_transform(&mut self, t: Matrice) {
        match self {
            Shape::Sphere(s) => s.set_transform(t),
            Shape::Plane(p) => p.set_transform(t),
            Shape::Cube(c) => c.transform = t,
        }
    }
    pub fn transform(&self) -> Matrice {
        match self {
            Shape::Sphere(s) => s.transform.clone(),
            Shape::Plane(p) => p.transform(),
            Shape::Cube(c) => c.transform.clone(),
        }
    }
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shape::Sphere(s) => s.normal_at(point),
            Shape::Plane(p) => p.normal_at(point),
            Shape::Cube(c) => c.normal_at(point),
        }
    }
    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material.clone(),
            Shape::Plane(p) => p.material.clone(),
            Shape::Cube(c) => c.material.clone(),
        }
    }
    pub fn set_material(&mut self, m: Material) {
        match self {
            Shape::Sphere(s) => s.material = m,
            Shape::Plane(_) => {}
            Shape::Cube(_) => {}
        }
    }
    pub fn set_material_ambient(&mut self, ambient: f32) {
        if let Shape::Sphere(s) = self {
            s.material.ambient = ambient
        }
    }
}

#[cfg(test)]
mod shape_tests {
    use std::f32::consts::PI;

    use crate::features::transformations::{rotation_z, scaling, translation};

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
        s.set_transform(translation(0.0, 1.0, 0.0));
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

    #[test]
    fn computing_normal_on_transformed_shape() {
        let mut s = test_shape();
        s.set_transform(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        let n = s.normal_at(Tuple::point(
            0.0,
            2.0_f32.sqrt() / 2.0,
            -(2.0_f32.sqrt() / 2.0),
        ));
        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254))
    }
}
