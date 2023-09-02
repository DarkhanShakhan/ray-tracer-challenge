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
        lightning(
            &comps.object.material(),
            &self.light,
            &comps.over_point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        )
    }
    pub fn color_at(&self, r: &Ray) -> Tuple {
        if let Some(i) = hit(intersect_world(self, r)) {
            let comps = Computation::new(&i, r);
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
        intersections::{computations::Computation, Intersection},
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
        let comps = Computation::new(&i, &r);
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
        let comps = Computation::new(&i, &r);
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
        let comps = Computation::new(&i, &r);
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
}
