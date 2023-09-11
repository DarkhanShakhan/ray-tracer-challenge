use std::fmt::Debug;

use super::{matrice::Matrice, shape::Shape, tuple::Tuple};

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
}

impl Pattern {
    pub fn at_object(&self, shape: &Shape, point: &Tuple) -> Tuple {
        let object_point = shape.transform().inverse().unwrap() * *point;
        let pattern_point = self.transform().inverse().unwrap() * object_point;
        match self {
            Pattern::Stripe(stripe) => stripe.at(&pattern_point),
            Pattern::Gradient(gradient) => gradient.at(&pattern_point),
            Pattern::Ring(ring) => ring.at(&pattern_point),
            Pattern::Checker(checker) => checker.at(&pattern_point),
        }
    }

    pub fn transform(&self) -> Matrice {
        match self {
            Pattern::Stripe(stripe) => stripe.transform.clone(),
            Pattern::Gradient(gradient) => gradient.transform.clone(),
            Pattern::Ring(ring) => ring.transform.clone(),
            Pattern::Checker(checker) => checker.transform.clone(),
        }
    }
    pub fn set_transform(&mut self, transform: Matrice) {
        match self {
            Pattern::Stripe(stripe) => stripe.set_transform(transform),
            Pattern::Gradient(gradient) => gradient.set_transform(transform),
            Pattern::Ring(ring) => ring.set_transform(transform),
            Pattern::Checker(checker) => checker.set_transform(transform),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Stripe {
    pub a: Tuple,
    pub b: Tuple,
    transform: Matrice,
}

impl Stripe {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        Self {
            a,
            b,
            transform: Matrice::identity_matrix(4),
        }
    }
    pub fn at(&self, point: &Tuple) -> Tuple {
        if point.x.floor() as i32 % 2 == 0 {
            return self.a;
        }
        self.b
    }
    pub fn at_object(&self, object: &Shape, point: &Tuple) -> Tuple {
        let object_point = object.transform().inverse().unwrap() * *point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;
        self.at(&pattern_point)
    }

    pub fn set_transform(&mut self, tranform: Matrice) {
        self.transform = tranform;
    }
}

#[cfg(test)]
mod patterns_tests {
    use crate::features::{
        spheres::Sphere,
        transformations::{scaling, translation},
    };

    use super::*;
    #[test]
    fn create_pattern() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Stripe::new(white, black);
        assert_eq!(p.a, white);
        assert_eq!(p.b, black);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Stripe::new(white, black);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 1.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Stripe::new(white, black);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 1.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 2.0)), white);
    }
    #[test]
    fn stripe_pattern_alternates_in_x() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Stripe::new(white, black);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(p.at(&Tuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(p.at(&Tuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(p.at(&Tuple::point(-1.1, 0.0, 0.0)), white);
    }
    #[test]
    fn stripes_with_object_transform() {
        let pattern = default_pattern();
        let object = {
            let mut object = Sphere::new();
            object.set_transform(scaling(2.0, 2.0, 2.0));
            object
        };
        let c = pattern.at_object(&Shape::Sphere(object), &Tuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, Tuple::color(1.0, 1.0, 1.0))
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = default_pattern();
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.at_object(&Shape::Sphere(object), &Tuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, Tuple::color(1.0, 1.0, 1.0))
    }

    #[test]
    fn stripes_with_pattern_and_object_transformation() {
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = default_pattern();
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = pattern.at_object(&Shape::Sphere(object), &Tuple::point(2.5, 0.0, 0.0));
        assert_eq!(c, Tuple::color(1.0, 1.0, 1.0))
    }

    fn default_pattern() -> Stripe {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        Stripe::new(white, black)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Gradient {
    from_color: Tuple,
    to_color: Tuple,
    transform: Matrice,
}

impl Gradient {
    pub fn new(from: Tuple, to: Tuple) -> Self {
        Self {
            from_color: from,
            to_color: to,
            transform: Matrice::identity_matrix(4),
        }
    }

    pub fn at(&self, point: &Tuple) -> Tuple {
        let distance = self.to_color - self.from_color;
        let fraction = point.x - point.x.floor();
        self.from_color + distance * fraction
    }
    pub fn set_transform(&mut self, tranform: Matrice) {
        self.transform = tranform;
    }
}

#[cfg(test)]
mod gradient_tests {
    use crate::features::tuple::Tuple;

    use super::Gradient;

    #[test]
    fn gradient_test() {
        let pattern = default_gradient();
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.25, 0.0, 0.0)),
            Tuple::color(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.5, 0.0, 0.0)),
            Tuple::color(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.75, 0.0, 0.0)),
            Tuple::color(0.25, 0.25, 0.25)
        );
    }

    fn default_gradient() -> Gradient {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        Gradient::new(white, black)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Ring {
    a: Tuple,
    b: Tuple,
    transform: Matrice,
}

impl Ring {
    pub fn new(a: Tuple, b: Tuple) -> Ring {
        Ring {
            a,
            b,
            transform: Matrice::identity_matrix(4),
        }
    }

    pub fn set_transform(&mut self, transform: Matrice) {
        self.transform = transform
    }
    pub fn transform(&self) -> Matrice {
        self.transform.clone()
    }
    pub fn at(&self, point: &Tuple) -> Tuple {
        if (point.x * point.x + point.z * point.z).sqrt() as i32 % 2 == 0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod ring_tests {
    use crate::features::tuple::Tuple;

    use super::Ring;

    #[test]
    fn test_ring() {
        let pattern = default_ring();
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(1.0, 0.0, 0.0)),
            Tuple::color(0.0, 0.0, 0.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 1.0)),
            Tuple::color(0.0, 0.0, 0.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.708, 0.0, 0.708)),
            Tuple::color(0.0, 0.0, 0.0)
        );
    }
    fn default_ring() -> Ring {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        Ring::new(white, black)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Checker {
    a: Tuple,
    b: Tuple,
    transform: Matrice,
}

impl Checker {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        Self {
            a,
            b,
            transform: Matrice::identity_matrix(4),
        }
    }
    pub fn set_transform(&mut self, transform: Matrice) {
        self.transform = transform
    }
    pub fn transform(&self) -> Matrice {
        self.transform.clone()
    }
    pub fn at(&self, point: &Tuple) -> Tuple {
        if (point.x.abs() + point.y.abs() + point.z.abs()) as i32 % 2 == 0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod checker_tests {

    use crate::features::tuple::Tuple;

    use super::Checker;

    #[test]
    fn test_checker_repeat_in_x() {
        let pattern = default_checker();
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.99, 0.0, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(1.01, 0.0, 0.0)),
            Tuple::color(0.0, 0.0, 0.0)
        );
    }
    #[test]
    fn test_checker_repeat_in_y() {
        let pattern = default_checker();
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.99, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 1.01, 0.0)),
            Tuple::color(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_checker_repeat_in_z() {
        let pattern = default_checker();
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 0.99)),
            Tuple::color(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.at(&Tuple::point(0.0, 0.0, 1.01)),
            Tuple::color(0.0, 0.0, 0.0)
        );
    }

    fn default_checker() -> Checker {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        Checker::new(white, black)
    }
}
