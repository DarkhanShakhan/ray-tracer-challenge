use std::f32::consts::PI;

use crate::features::{
    camera::cameras::Camera,
    lights::Light,
    materials::Material,
    patterns::{Checker, Gradient, Pattern, Ring, Stripe},
    planes::Plane,
    shape::Shape,
    spheres::Sphere,
    transformations::{
        rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transformation,
    },
    tuple::Tuple,
    world::World,
};

pub fn draw_sphere() {
    let light_position = Tuple::point(-7.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    let floor = Plane::new();
    let mut left_wall = Plane::new();
    left_wall.set_transform(translation(0.0, 10.0, 0.0) * rotation_z(PI / 2.0));
    let mut right_wall = Plane::new();
    right_wall.set_transform(translation(0.0, 10.0, 0.0) * rotation_z(-PI / 2.0));

    let mut middle = Sphere::new();
    middle.transform = translation(1.5, 1.0, 0.5);
    middle.material = Material::new();
    middle.material.pattern = Some(Pattern::Gradient(Gradient::new(
        Tuple::color(0.0, 0.0, 0.0),
        Tuple::color(1.0, 1.0, 1.0),
    )));
    // middle.material.color = Tuple::color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    // right.transform = translation(-0.5, 1.0, 0.0)
    //     * scaling(0.5, 0.5, 0.5)
    //     * shearing(-0.2, -1.5, -1.5, -1.5, -1.5, -1.5);
    right.material = Material::new();
    right.material.pattern = Some(Pattern::Checker(Checker::new(
        Tuple::color(1.0, 1.0, 1.0),
        Tuple::color(0.0, 0.0, 0.0),
    )));
    right.material.color = Tuple::color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = Material::new();
    left.material.color = Tuple::color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let world = World::new(
        light.clone(),
        &[
            Shape::Plane(left_wall),
            Shape::Plane(right_wall),
            Shape::Plane(floor),
            Shape::Sphere(middle),
            Shape::Sphere(right),
            Shape::Sphere(left),
        ],
    );
    let mut camera = Camera::new(400.0, 200.0, PI / 2.0);
    camera.transform = view_transformation(
        Tuple::point(0.0, 1.0, -8.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 2.0, 0.0),
    );
    camera.render(&world).to_ppm();
}
