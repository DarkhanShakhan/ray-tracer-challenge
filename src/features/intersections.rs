use super::shape::Shape;
#[derive(Clone, PartialEq, Debug)]
pub struct Intersection {
    pub t: f32,
    pub s: Shape,
}

impl Intersection {
    pub fn new(t: f32, s: Shape) -> Self {
        Self { t, s }
    }
}

pub fn intersections(xs: &mut [Intersection]) -> Vec<Intersection> {
    xs.sort_by(|a, b| a.t.total_cmp(&b.t));
    xs.to_vec()
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
    use crate::features::shape::Shape;
    use crate::features::spheres::Sphere;
    use crate::features::{rays::Ray, tuple::Tuple};

    use super::Intersection;

    #[test]
    fn test_creating_intersection() {
        let s = Shape::Sphere(Sphere::new());
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.s, s);
    }

    #[test]
    fn test_intersecting_sphere_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::Sphere(Sphere::new());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[0].s, s);
    }

    #[test]
    fn test_intersecting_sphere_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::Sphere(Sphere::new());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
        assert_eq!(xs[0].s, s);
    }
    #[test]
    fn test_no_intersection() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::Sphere(Sphere::new());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn test_intersecting_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::Sphere(Sphere::new());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
        assert_eq!(xs[0].s, s);
    }
    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::Sphere(Sphere::new());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}

#[cfg(test)]
mod intersect_transformed_sphere_test {
    use crate::features::{
        rays::Ray,
        shape::Shape,
        spheres::Sphere,
        transformations::{scaling, translation},
        tuple::Tuple,
    };

    #[test]
    fn test_scaled_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Shape::Sphere(Sphere::new());
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_translated_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Shape::Sphere(Sphere::new());
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::features::{
        intersections::{hit, intersections},
        shape::Shape,
        spheres::Sphere,
    };

    use super::Intersection;

    #[test]
    fn test_all_intersections_have_positive_t() {
        let s = Shape::Sphere(Sphere::new());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let xs = intersections(&mut [i2, i1.clone()]);
        let i = hit(xs).unwrap();
        assert!(i == i1.clone());
    }
    #[test]
    fn test_some_intersections_have_negative_t() {
        let s = Shape::Sphere(Sphere::new());
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let xs = intersections(&mut [i2.clone(), i1]);
        let i = hit(xs).unwrap();
        assert!(i == i2);
    }
    #[test]
    fn test_all_intersections_have_negative_t() {
        let s = Shape::Sphere(Sphere::new());
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let xs = intersections(&mut [i2, i1]);
        let i = hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn test_lowest_non_negative_t() {
        let s = Shape::Sphere(Sphere::new());
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

    use crate::features::{rays::Ray, shape::Shape, tuple::Tuple};

    use super::{hit, Intersection};

    pub struct Computation {
        pub t: f32,
        pub object: Shape,
        pub point: Tuple,
        pub eyev: Tuple,
        pub normalv: Tuple,
        pub reflectv: Tuple,
        pub inside: bool,
        pub over_point: Tuple,
        pub under_point: Tuple,
        pub n1: f32,
        pub n2: f32,
    }

    impl Computation {
        pub fn new(i: &Intersection, r: &Ray, xs: &[Intersection]) -> Self {
            let mut n1 = 1.0;
            let mut n2 = 1.0;
            let mut containers: Vec<Shape> = vec![];
            for x in xs.iter() {
                if *i == *x {
                    if let Some(l) = containers.last() {
                        n1 = l.material().refractive_index;
                    }
                }
                if let Some(index) = containers.iter().position(|a| *a == x.s) {
                    containers.remove(index);
                } else {
                    containers.push(x.s.clone())
                }
                if *i == *x {
                    if let Some(l) = containers.last() {
                        n2 = l.material().refractive_index;
                    }
                    break;
                }
            }
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
                reflectv: r.direction.reflect(&normalv),
                inside,
                over_point: r.position(i.t) + (normalv * 0.0001),
                under_point: r.position(i.t) - (normalv * 0.0001),
                n1,
                n2,
            }
        }
    }
    #[cfg(test)]
    mod computation_tests {
        use std::f32::EPSILON;

        use crate::features::{
            intersections::Intersection, rays::Ray, shape::Shape, spheres::Sphere,
            transformations::translation, tuple::Tuple,
        };

        use super::Computation;

        #[test]
        fn test_prepare_computation() {
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let s = Shape::Sphere(Sphere::new());
            let i = Intersection::new(4.0, s.clone());
            let comps = Computation::new(&i, &r, &[]);
            assert_eq!(comps.t, i.t);
            assert_eq!(comps.object, i.s);
            assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
            assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
            assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        }
        #[test]
        fn test_hit_outside() {
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let s = Shape::Sphere(Sphere::new());
            let i = Intersection::new(4.0, s.clone());
            let comps = Computation::new(&i, &r, &[]);
            assert!(!comps.inside);
        }
        #[test]
        fn test_hit_inside() {
            let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
            let s = Shape::Sphere(Sphere::new());
            let i = Intersection::new(1.0, s.clone());
            let comps = Computation::new(&i, &r, &[]);
            assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
            assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
            assert!(comps.inside);
            assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0))
        }
        #[test]
        fn test_hit_offset_point() {
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let mut s = Shape::Sphere(Sphere::new());
            s.set_transform(translation(0.0, 0.0, 1.0));
            let i = Intersection::new(5.0, s.clone());
            let comps = Computation::new(&i, &r, &[]);
            assert!(comps.over_point.z < -EPSILON / 2.0);
            assert!(comps.point.z > comps.over_point.z)
        }
    }
}

#[cfg(test)]
mod reflectv_tests {
    use crate::features::{planes::Plane, rays::Ray, tuple::Tuple};

    use super::{computations::Computation, *};
    #[test]
    fn test_precomputing_reflection_vector() {
        let shape = Shape::Plane(Plane::new());
        let r = Ray::new(
            Tuple::point(0.0, 1.0, -1.0),
            Tuple::vector(0.0, -(2.0_f32.sqrt() / 2.0), 2.0_f32.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f32.sqrt(), shape);
        let comps = Computation::new(&i, &r, &[]);
        assert_eq!(
            comps.reflectv,
            Tuple::vector(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0)
        )
    }
}

#[cfg(test)]
mod reflection_refraction_tests {
    use std::f32::EPSILON;

    use crate::features::{
        rays::Ray,
        spheres::Sphere,
        transformations::{scaling, translation},
        tuple::Tuple,
    };

    use super::{computations::Computation, *};
    #[test]
    fn test_n1_n2() {
        let mut a = Sphere::glass_sphere();
        a.transform = scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;
        let mut b = Sphere::glass_sphere();
        b.transform = translation(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;
        let mut c = Sphere::glass_sphere();
        c.transform = translation(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;
        let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersections(&mut [
            Intersection::new(2.0, Shape::Sphere(a.clone())),
            Intersection::new(2.75, Shape::Sphere(b.clone())),
            Intersection::new(3.25, Shape::Sphere(c.clone())),
            Intersection::new(4.75, Shape::Sphere(b.clone())),
            Intersection::new(5.25, Shape::Sphere(c.clone())),
            Intersection::new(6.0, Shape::Sphere(a.clone())),
        ]);
        let results = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for (ix, x) in xs.iter().enumerate() {
            let comps = Computation::new(x, &r, &xs);
            assert_eq!(comps.n1, results[ix].0);
            assert_eq!(comps.n2, results[ix].1);
        }
    }

    #[test]
    fn test_under_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::glass_sphere();
        shape.transform = translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, Shape::Sphere(shape));
        let xs = intersections(&mut [i.clone()]);
        let comps = Computation::new(&i, &r, &xs);
        assert!(comps.under_point.z > 0.0001 / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}
