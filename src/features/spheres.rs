use uuid::Uuid;

use super::{
    intersections::{intersections, Intersection},
    materials::Material,
    matrice::Matrice,
    rays::{transform, Ray},
    shape::Shape,
    tuple::{Tuple, TupleType},
};

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrice,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            material: Material::new(),
            id: Uuid::new_v4(),
            transform: Matrice::identity_matrix(4),
        }
    }
}
impl Shape for Sphere {
    fn get_material(&self) -> Material {
        self.material.clone()
    }
    fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    fn set_transform(&mut self, transformation: Matrice) {
        self.transform = transformation;
    }
    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let r2 = transform(r.clone(), self.transform.inverse().unwrap());
        let sphere_to_ray = r2.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = r2.direction.dot(&r2.direction);
        let b = 2.0 * r2.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let disrciminant = b * b - 4.0 * a * c;
        if disrciminant < 0.0 {
            return vec![];
        }
        intersections(&mut [
            Intersection::new((-b - disrciminant.sqrt()) / (2.0 * a), self.clone()),
            Intersection::new((-b + disrciminant.sqrt()) / (2.0 * a), self.clone()),
        ])
    }
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = TupleType::Vector;
        world_normal.normalize()
    }
}
