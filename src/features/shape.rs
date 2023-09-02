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
