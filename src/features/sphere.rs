use super::rays::Ray;

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn intersect(s: &Sphere, r: &Ray) -> Vec<f32> {
    vec![]
}

#[cfg(test)]
mod sphere_tests {
    use crate::features::{rays::Ray, tuple::Tuple};

    use super::{intersect, Sphere};

    #[test]
    fn test_intersecting_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }
}
