use std::f32::consts::PI;

use crate::features::{
    camera::cameras::Camera,
    lights::Light,
    materials::Material,
    spheres::Sphere,
    transformations::{
        rotation_x, rotation_y, scaling, shearing, translation, view_transformation,
    },
    tuple::Tuple,
    world::World,
};

pub fn draw_sphere() {
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    let mut floor = Sphere::new();
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material = Material::new();
    floor.material.color = Tuple::color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut middle = Sphere::new();
    middle.transform = translation(1.5, 1.0, 0.5);
    middle.material = Material::new();
    middle.material.color = Tuple::color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = translation(-0.5, 1.0, 0.0)
        * scaling(0.5, 0.5, 0.5)
        * shearing(-0.2, -1.5, -1.5, -1.5, -1.5, -1.5);
    right.material = Material::new();
    right.material.color = Tuple::color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = Material::new();
    left.material.color = Tuple::color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let world = World::new(light.clone(), &[floor, middle, right, left]);
    let mut camera = Camera::new(1200.0, 600.0, PI / 3.0);
    camera.transform = view_transformation(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );
    camera.render(&world).to_ppm();
}
