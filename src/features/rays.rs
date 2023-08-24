use super::{matrice::Matrice, tuple::Tuple};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}
impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }
    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }
}

pub fn transform(r: &Ray, m: Matrice) -> Ray {
    Ray::new(m.clone() * r.origin, m.clone() * r.direction)
}
#[cfg(test)]
mod ray_tests {
    use crate::features::tuple::Tuple;

    use super::Ray;

    #[test]
    fn test_creating_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn test_ray_position() {
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        println!("{:?}", ray.position(2.5));
        assert!(ray.position(0.0) == origin);
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}

#[cfg(test)]
mod ray_transformation_tests {
    use crate::features::{
        transformations::{scaling, translation},
        tuple::Tuple,
    };

    use super::{transform, Ray};

    #[test]
    fn test_translating_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = transform(&r, m);
        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = transform(&r, m);
        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }
}
