use super::tuple::Tuple;

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
