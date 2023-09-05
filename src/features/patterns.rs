use super::{shape::Shape, tuple::Tuple};

#[derive(PartialEq, Debug, Clone)]
pub struct Pattern {
    pub a: Tuple,
    pub b: Tuple,
}

impl Pattern {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        Self { a, b }
    }
    pub fn at(&self, point: &Tuple) -> Tuple {
        if point.x.floor() as i32 % 2 == 0 {
            return self.a;
        }
        self.b
    }
    pub fn at_object(&self, object: &Shape, point: &Tuple) -> Tuple {
        self.a
    }
}

#[cfg(test)]
mod patterns_tests {
    use super::*;
    #[test]
    fn create_pattern() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Pattern::new(white, black);
        assert_eq!(p.a, white);
        assert_eq!(p.b, black);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Pattern::new(white, black);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 1.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Pattern::new(white, black);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 1.0)), white);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 2.0)), white);
    }
    #[test]
    fn stripe_pattern_alternates_in_x() {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        let p = Pattern::new(white, black);
        assert_eq!(p.at(&Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(p.at(&Tuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(p.at(&Tuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(p.at(&Tuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(p.at(&Tuple::point(-1.1, 0.0, 0.0)), white);
    }
    #[test]
    fn stripes_with_object_transform() {
        let p = default_pattern();
    }

    fn default_pattern() -> Pattern {
        let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
        let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
        Pattern::new(white, black)
    }
}
