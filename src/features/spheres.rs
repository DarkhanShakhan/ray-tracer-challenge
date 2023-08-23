use uuid::Uuid;

use super::{matrice::Matrice, rays::Ray, tuple::Tuple};

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
