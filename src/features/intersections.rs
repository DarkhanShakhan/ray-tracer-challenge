use crate::features::tuple::Tuple;

use super::{
    rays::{transform, Ray},
    spheres::Sphere,
};
#[derive(Clone, PartialEq, Debug)]
pub struct Intersection {
    pub t: f32,
    pub s: Sphere,
}

impl Intersection {
    pub fn new(t: f32, s: Sphere) -> Self {
        Self { t, s }
    }
}

pub fn intersections(xs: &mut [Intersection]) -> Vec<Intersection> {
    xs.sort_by(|a, b| a.t.total_cmp(&b.t));
    xs.to_vec()
}

pub fn intersect(s: &Sphere, r: &Ray) -> Vec<Intersection> {
    let r2 = transform(r.clone(), s.transform.inverse().unwrap());
    let sphere_to_ray = r2.origin - Tuple::point(0.0, 0.0, 0.0);
    let a = r2.direction.dot(&r2.direction);
    let b = 2.0 * r2.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
    let disrciminant = b * b - 4.0 * a * c;
    if disrciminant < 0.0 {
        return vec![];
    }
    intersections(&mut [
        Intersection::new((-b - disrciminant.sqrt()) / (2.0 * a), s.clone()),
        Intersection::new((-b + disrciminant.sqrt()) / (2.0 * a), s.clone()),
    ])
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    for i in xs {
        if i.t > 0.0 {
            return Some(i.clone());
        }
    }
    None
}

#[cfg(test)]
mod intersection_tests {
    use super::intersect;
    use crate::features::spheres::Sphere;
    use crate::features::{rays::Ray, tuple::Tuple};

    use super::Intersection;

    #[test]
    fn test_creating_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.s, s);
    }

    #[test]
    fn test_intersecting_sphere_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[0].s, s);
    }

    #[test]
    fn test_intersecting_sphere_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
        assert_eq!(xs[0].s, s);
    }
    #[test]
    fn test_no_intersection() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn test_intersecting_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
        assert_eq!(xs[0].s, s);
    }
    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}

#[cfg(test)]
mod intersect_transformed_sphere_test {
    use crate::features::{
        rays::Ray,
        spheres::Sphere,
        transformations::{scaling, translation},
        tuple::Tuple,
    };

    use super::intersect;

    #[test]
    fn test_scaled_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_translated_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::features::{
        intersections::{hit, intersections},
        spheres::Sphere,
    };

    use super::Intersection;

    #[test]
    fn test_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let xs = intersections(&mut [i2, i1.clone()]);
        let i = hit(xs).unwrap();
        assert!(i == i1.clone());
    }
    #[test]
    fn test_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let xs = intersections(&mut [i2.clone(), i1]);
        let i = hit(xs).unwrap();
        assert!(i == i2);
    }
    #[test]
    fn test_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let xs = intersections(&mut [i2, i1]);
        let i = hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn test_lowest_non_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());
        let xs = intersections(&mut [i1, i2, i3, i4.clone()]);
        let i = hit(xs).unwrap();
        assert!(i == i4);
    }
}
pub mod computations {
    use std::f32::EPSILON;

    use crate::features::{rays::Ray, spheres::Sphere, tuple::Tuple};

    use super::Intersection;

    pub struct Computation {
        pub t: f32,
        pub object: Sphere,
        pub point: Tuple,
        pub eyev: Tuple,
        pub normalv: Tuple,
        pub inside: bool,
        pub over_point: Tuple,
    }

    impl Computation {
        pub fn new(i: &Intersection, r: &Ray) -> Self {
            let mut normalv = i.s.normal_at(r.position(i.t));
            let mut inside = false;
            if normalv.dot(&-(r.direction)) < 0.0 {
                inside = true;
                normalv = -normalv;
            }
            Computation {
                t: i.t,
                object: i.s.clone(),
                point: r.position(i.t),
                eyev: -(r.direction),
                normalv,
                inside,
                over_point: r.position(i.t) + normalv * 0.0001,
            }
        }
    }
    #[cfg(test)]
    mod computation_tests {
        use std::f32::EPSILON;

        use crate::features::{
            intersections::Intersection, rays::Ray, spheres::Sphere, transformations::translation,
            tuple::Tuple,
        };

        use super::Computation;

        #[test]
        fn test_prepare_computation() {
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let shape = Sphere::new();
            let i = Intersection::new(4.0, shape.clone());
            let comps = Computation::new(&i, &r);
            assert_eq!(comps.t, i.t);
            assert_eq!(comps.object, i.s);
            assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
            assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
            assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        }
        #[test]
        fn test_hit_outside() {
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let shape = Sphere::new();
            let i = Intersection::new(4.0, shape.clone());
            let comps = Computation::new(&i, &r);
            assert!(!comps.inside);
        }
        #[test]
        fn test_hit_inside() {
            let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
            let shape = Sphere::new();
            let i = Intersection::new(1.0, shape.clone());
            let comps = Computation::new(&i, &r);
            assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
            assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
            assert!(comps.inside);
            assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0))
        }
        #[test]
        fn test_hit_offset_point() {
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let mut shape = Sphere::new();
            shape.transform = translation(0.0, 0.0, 1.0);
            let i = Intersection::new(5.0, shape.clone());
            let comps = Computation::new(&i, &r);
            assert!(comps.over_point.z < -EPSILON / 2.0);
            assert!(comps.point.z > comps.over_point.z)
        }
    }
}
