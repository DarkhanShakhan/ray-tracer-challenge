use crate::features::{rays::transform, shape::Shape};

use super::{
    intersections::Intersection, materials::Material, matrice::Matrice, rays::Ray, tuple::Tuple,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Plane {
    pub material: Material,
    pub transform: Matrice,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            material: Material::new(),
            transform: Matrice::identity_matrix(4),
        }
    }

    pub fn normal_at(&self, _point: Tuple) -> Tuple {
        self.transform.clone() * Tuple::vector(0.0, 1.0, 0.0)
    }

    pub fn set_transform(&mut self, transform: Matrice) {
        self.transform = transform
    }

    pub fn transform(&self) -> Matrice {
        self.transform.clone()
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // let r = transform(ray.clone(), self.transform.clone());
        if ray.direction.y.abs() < 0.00001 {
            return vec![];
        }
        let t = (-ray.origin.y) / ray.direction.y;
        vec![Intersection::new(t, Shape::Plane(self.clone()))]
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::features::shape::Shape;

    use super::*;
    #[test]
    fn constant_normal_everywhere() {
        let p = Plane::new();
        assert_eq!(
            p.normal_at(Tuple::point(0.0, 0.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            p.normal_at(Tuple::point(10.0, 0.0, -10.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            p.normal_at(Tuple::point(-5.0, 0.0, 150.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0)
    }

    #[test]
    fn intersect_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].s, Shape::Plane(p))
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].s, Shape::Plane(p))
    }
}
