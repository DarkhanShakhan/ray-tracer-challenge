use indicatif::ProgressBar;

use crate::features::{
    canvas::Canvas,
    intersections::{hit, intersect},
    lights::Light,
    materials::lightning,
    rays::Ray,
    spheres::Sphere,
    transformations::rotation_x,
    tuple::Tuple,
};

pub fn draw_sphere() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_size = 7.0;
    let canvas_pixels = 500.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let mut shape = Sphere::new();
    shape.material.color = Tuple::color(1.0, 0.2, 1.0);
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    let mut color = Tuple::default_color();
    let bar = ProgressBar::new(canvas.height as u64);
    for y in 0..canvas_pixels as usize {
        let world_y = half - (pixel_size * y as f32);
        for x in 0..canvas_pixels as usize {
            let world_x = -half + (pixel_size * x as f32);
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            let xs = intersect(&shape, &ray);
            if let Some(hit) = hit(xs) {
                let point = ray.position(hit.t);
                let normal = hit.s.normal_at(point);
                let eye = -ray.direction;
                color = lightning(&hit.s.material, &light, &position, &eye, &normal);
            }

            canvas.write_pixel(x, y, color);
            color = Tuple::default_color();
        }
        bar.inc(1);
    }
    canvas.to_ppm();
}
