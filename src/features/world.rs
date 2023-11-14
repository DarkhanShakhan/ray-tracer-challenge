use super::{
    intersections::{computations::Computation, hit, intersections, Intersection},
    lights::Light,
    materials::{lightning, Material},
    rays::Ray,
    shape::Shape,
    spheres::Sphere,
    transformations::scaling,
    tuple::Tuple,
};

pub struct World {
    pub light: Light,
    pub shapes: Vec<Shape>,
}

impl World {
    pub fn new(light: Light, shapes: &[Shape]) -> Self {
        Self {
            light,
            shapes: shapes.to_vec(),
        }
    }
    pub fn set_light(&mut self, light: Light) {
        self.light = light;
    }
    pub fn shade_hit(&self, comps: &Computation) -> Tuple {
        let shadowed = self.is_shadowed(&comps.over_point);
        let surface = lightning(
            &comps.object.material(),
            &comps.object,
            &self.light,
            &comps.over_point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(comps);
        surface + reflected
    }
    pub fn color_at(&self, r: &Ray) -> Tuple {
        if let Some(i) = hit(intersect_world(self, r)) {
            let comps = Computation::new(&i, r, &intersect_world(self, r));
            return self.shade_hit(&comps);
        }
        Tuple::default_color()
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        let intersections = intersect_world(self, &r);
        if let Some(h) = hit(intersections) {
            if h.t < distance {
                return true;
            }
        }
        false
    }
    pub fn reflected_color(&self, comps: &Computation) -> Tuple {
        if comps.object.material().reflective == 0.0 {
            return Tuple::default_color();
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray);
        color * comps.object.material().reflective
    }
    pub fn refracted_color(&self, comps: &Computation, remaining: u32) -> Tuple {
        if comps.object.material().transparency == 0.0 || remaining == 0 {
            return Tuple::color(0.0, 0.0, 0.0);
        }
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(&comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return Tuple::color(0.0, 0.0, 0.0);
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);
        self.color_at(&refract_ray) * comps.object.material().transparency
    }
}

pub fn intersect_world(world: &World, ray: &Ray) -> Vec<Intersection> {
    let mut out = vec![];
    for ix in 0..world.shapes.len() {
        let mut xs = world.shapes[ix].intersect(ray);
        out.append(&mut xs);
    }
    intersections(&mut out)
}
impl Default for World {
    fn default() -> Self {
        let light = Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        let mut s1 = Shape::Sphere(Sphere::new());
        let mut material = Material::new();
        material.color = Tuple::color(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        s1.set_material(material);
        let mut s2 = Shape::Sphere(Sphere::new());
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        let shapes = &[s1, s2];
        Self::new(light, shapes)
    }
}

#[cfg(test)]
mod world_tests {
    use crate::features::{
        intersections::{computations::Computation, intersections, Intersection},
        lights::Light,
        rays::Ray,
        shape::Shape,
        spheres::Sphere,
        transformations::translation,
        tuple::Tuple,
    };

    use super::{intersect_world, World};

    #[test]
    fn testing_default_world() {
        let d_w = World::default();
        let light = Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        assert_eq!(d_w.light, light);
    }

    #[test]
    fn testing_intersect_world() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersect_world(&w, &r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn testing_shading_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.shapes[0].clone();
        let i = Intersection::new(4.0, shape);
        let comps = Computation::new(&i, &r, &[]);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn testing_shading_intersection_inside() {
        let mut w = World::default();
        w.set_light(Light::new(
            Tuple::point(0.0, 0.25, 0.0),
            Tuple::color(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.shapes[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = Computation::new(&i, &r, &[]);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Tuple::color(0.9049522, 0.9049522, 0.9049522))
    }
    #[test]
    fn testing_shade_hit_given_intersection_in_shadow() {
        let mut w = World::default();
        w.set_light(Light::new(
            Tuple::point(0.0, 0.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        ));
        let s1 = Shape::Sphere(Sphere::new());
        let mut s2 = Shape::Sphere(Sphere::new());
        s2.set_transform(translation(0.0, 0.0, 10.0));
        w.shapes = vec![s1, s2.clone()];
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);
        let comps = Computation::new(&i, &r, &[]);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Tuple::color(0.1, 0.1, 0.1));
    }

    #[cfg(test)]
    mod world_color_tests {
        use crate::features::{rays::Ray, tuple::Tuple, world::World};

        #[test]
        fn testing_color_ray_miss() {
            let w = World::default();
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
            let c = w.color_at(&r);
            assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
        }

        #[test]
        fn testing_color_ray_hits() {
            let w = World::default();
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let c = w.color_at(&r);
            assert_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855));
        }

        // #[test]
        // fn testing_color_intersection_behind_ray() {
        //     let mut w = World::default();
        //     w.shapes[0].material.ambient = 1.0;
        //     w.shapes[1].material.ambient = 1.0;
        //     let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        //     let c = w.color_at(&r);
        //     assert_eq!(c, w.shapes[1].material.color);
        // }
    }

    #[cfg(test)]
    mod is_shadowed_tests {
        use super::*;
        #[test]
        fn testing_no_shadow() {
            let world = World::default();
            let point = Tuple::point(0.0, 10.0, 0.0);
            assert!(!world.is_shadowed(&point))
        }

        #[test]
        fn testing_object_between_point_light() {
            let world = World::default();
            let point = Tuple::point(10.0, -10.0, 10.0);
            assert!(world.is_shadowed(&point))
        }

        #[test]
        fn testing_object_behind_light() {
            let world = World::default();
            let point = Tuple::point(-20.0, 10.0, -20.0);
            assert!(!world.is_shadowed(&point))
        }
        #[test]
        fn testing_object_behind_point() {
            let world = World::default();
            let point = Tuple::point(-2.0, 2.0, -2.0);
            assert!(!world.is_shadowed(&point))
        }
    }
    #[cfg(test)]
    mod reflected_color_tests {
        use crate::features::planes::Plane;

        use super::*;
        #[test]
        fn testing_reflected_color_for_nonreflective_material() {
            let mut world = World::default();
            let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
            world.shapes[1].set_material_ambient(1.0);
            let i = Intersection::new(1.0, world.shapes[1].clone());
            let comps = Computation::new(&i, &r, &[]);
            let color = world.reflected_color(&comps);
            assert_eq!(color, Tuple::color(0.0, 0.0, 0.0))
        }
        #[test]
        fn testing_reflected_color_for_reflective_material() {
            let mut world = World::default();
            let mut plane = Plane::new();
            plane.material.reflective = 0.5;
            plane.transform = translation(0.0, -1.0, 0.0);
            let shape = Shape::Plane(plane);
            world.shapes.push(shape.clone());
            let r = Ray::new(
                Tuple::point(0.0, 0.0, -3.0),
                Tuple::vector(0.0, -(2.0_f32.sqrt() / 2.0), 2.0_f32.sqrt() / 2.0),
            );
            let i = Intersection::new(2.0_f32.sqrt(), shape);
            let comps = Computation::new(&i, &r, &[]);
            let color = world.reflected_color(&comps);
            assert_eq!(color, Tuple::color(0.19032, 0.2379, 0.14274));
        }
        #[test]
        fn testing_shade_hit_with_reflective_material() {
            let mut world = World::default();
            let mut shape = Plane::new();
            shape.material.reflective = 0.5;
            shape.transform = translation(0.0, -1.0, 0.0);
            world.shapes.push(Shape::Plane(shape.clone()));
            let r = Ray::new(
                Tuple::point(0.0, 0.0, -3.0),
                Tuple::vector(0.0, -(2.0_f32.sqrt() / 2.0), 2.0_f32.sqrt() / 2.0),
            );
            let i = Intersection::new(2.0_f32.sqrt(), Shape::Plane(shape));
            let comps = Computation::new(&i, &r, &[]);
            let color = world.shade_hit(&comps);
            assert_eq!(color, Tuple::color(0.87677, 0.92436, 0.82918));
        }
        #[test]
        fn testing_color_at_with_mutually_reflective_surfaces() {
            let mut world = World {
                light: Light::new(Tuple::point(0.0, 0.0, 0.0), Tuple::color(1.0, 1.0, 1.0)),
                ..Default::default()
            };
            let mut lower = Plane::new();
            lower.material.reflective = 1.0;
            lower.transform = translation(0.0, -1.0, 0.0);
            world.shapes.push(Shape::Plane(lower.clone()));
            let mut upper = Plane::new();
            upper.material.reflective = 1.0;
            upper.transform = translation(0.0, 1.0, 0.0);
            world.shapes.push(Shape::Plane(upper.clone()));
            let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

            world.color_at(&r);
        }
    }

    #[cfg(test)]
    mod refracted_color_tests {
        use crate::features::patterns::{Checker, Pattern, Stripe};

        use super::*;
        #[test]
        fn test_refracted_color_with_opaque_surface() {
            let w = World::default();
            let shape = &w.shapes[0];
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let xs = intersections(&mut [
                Intersection::new(4.0, shape.clone()),
                Intersection::new(6.0, shape.clone()),
            ]);
            let comps = Computation::new(&xs[0], &r, &xs);
            let c = w.refracted_color(&comps, 5);
            assert_eq!(c, Tuple::color(0.0, 0.0, 0.0))
        }

        #[test]
        fn test_refracted_color_at_max_recursive_depth() {
            let mut w = World::default();
            let shape = &mut w.shapes[0];
            let mut m = shape.material();
            m.transparency = 1.0;
            m.refractive_index = 1.5;
            shape.set_material(m);
            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let xs = intersections(&mut [
                Intersection::new(4.0, shape.clone()),
                Intersection::new(6.0, shape.clone()),
            ]);
            let comps = Computation::new(&xs[0], &r, &xs);
            let c = w.refracted_color(&comps, 0);
            assert_eq!(c, Tuple::color(0.0, 0.0, 0.0))
        }

        #[test]
        fn test_refracted_color_under_total_internal_reflection() {
            let mut w = World::default();
            let shape = &mut w.shapes[0];
            let mut m = shape.material();
            m.transparency = 1.0;
            m.refractive_index = 1.5;
            shape.set_material(m);
            let r = Ray::new(
                Tuple::point(0.0, 0.0, 2_f32.sqrt() / 2.0),
                Tuple::vector(0.0, 1.0, 0.0),
            );
            let xs = intersections(&mut [
                Intersection::new(-(2_f32.sqrt() / 2.0), shape.clone()),
                Intersection::new(2_f32.sqrt() / 2.0, shape.clone()),
            ]);
            let comps = Computation::new(&xs[1], &r, &xs);
            let c = w.refracted_color(&comps, 5);
            assert_eq!(c, Tuple::color(0.0, 0.0, 0.0))
        }

        #[test]
        fn refracted_color_with_refracted_ray() {
            let mut w = World::default();
            let mut a = w.shapes[0].clone();
            let mut m = a.material();
            m.ambient = 1.0;
            m.pattern = Some(Pattern::Stripe(default_pattern()));
            a.set_material(m);
            let mut b = w.shapes[1].clone();
            m = b.material();
            m.transparency = 1.0;
            m.refractive_index = 1.5;
            b.set_material(m);
            w.shapes[0] = a.clone();
            w.shapes[1] = b.clone();
            let r = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));
            let xs = intersections(&mut [
                Intersection::new(-0.9899, a.clone()),
                Intersection::new(-0.4899, b.clone()),
                Intersection::new(0.4899, b.clone()),
                Intersection::new(0.9899, a.clone()),
            ]);
            let comps = Computation::new(&xs[1], &r, &xs);
            let c = w.refracted_color(&comps, 5);
            assert_eq!(c, Tuple::color(0.0, 0.99888, 0.04725));
        }
        fn default_pattern() -> Stripe {
            let white: Tuple = Tuple::color(1.0, 1.0, 1.0);
            let black: Tuple = Tuple::color(0.0, 0.0, 0.0);
            Stripe::new(white, black)
        }
    }
}
