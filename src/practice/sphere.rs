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
    let light_position = Tuple::point(-8.0, 2.0, -28.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    let mut floor = Plane::new();
    floor.material.pattern = Some(Pattern::Checker(Checker::new(
        Tuple::color(0.0, 0.0, 0.0),
        Tuple::color(1.0, 1.0, 1.0),
    )));
    let mut left_wall = Plane::new();
    left_wall
        .set_transform(translation(0.0, 0.0, 8.0) * rotation_y(PI / 5.0) * rotation_x(PI / 2.0));
    left_wall.material.color = Tuple::color(0.5, 0.5, 0.5);
    let mut ceiling = Plane::new();
    ceiling.set_transform(translation(0.0, 5.0, 0.0));
    ceiling.material.color = Tuple::color(0.7, 0.8, 0.4);
    let mut right_wall = Plane::new();
    right_wall
        .set_transform(translation(0.0, 0.0, 8.0) * rotation_y(-PI / 5.0) * rotation_x(PI / 2.0));
    right_wall.material.color = Tuple::color(0.75, 0.75, 0.75);
    let mut middle = Sphere::new();
    middle.transform = translation(2.0, 2.0, 2.5) * scaling(2.0, 2.0, 2.0);
    middle.material = Material::new();
    // middle.material.pattern = Some(Pattern::Gradient(Gradient::new(
    //     Tuple::color(0.0, 0.0, 0.0),
    //     Tuple::color(1.0, 1.0, 1.0),
    // )));
    // middle.material.color = Tuple::color(0.1, 1.0, 0.5);
    middle.material.reflective = 0.5;
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = translation(-2.5, 1.5, 4.0) * scaling(1.5, 1.5, 1.5);
    right.material = Material::new();
    right.material.reflective = 0.8;
    right.material.color = Tuple::color(0.7, 0.8, 0.4);
    // right.material.pattern = Some(Pattern::Checker(Checker::new(
    //     Tuple::color(1.0, 1.0, 1.0),
    //     Tuple::color(0.0, 0.0, 0.0),
    // )));
    // right.material.color = Tuple::color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = translation(-2.5, 0.33, 0.75) * scaling(0.33, 0.33, 0.33);
    left.material = Material::new();
    left.material.color = Tuple::color(1.0, 0.8, 0.1);
    // left.material.reflective = 0.3;
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let world = World::new(
        light.clone(),
        &[
            Shape::Plane(left_wall),
            Shape::Plane(right_wall),
            Shape::Plane(ceiling),
            Shape::Plane(floor),
            Shape::Sphere(middle),
            Shape::Sphere(right),
            Shape::Sphere(left),
        ],
    );
    let mut camera = Camera::new(800.0, 400.0, PI / 2.0);
    // let mut camera = Camera::new(140.0, 70.0, PI / 2.0);

    camera.transform = view_transformation(
        Tuple::point(0.0, 1.0, -8.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 2.0, 0.0),
    );
    camera.render(&world).to_ppm();
}
