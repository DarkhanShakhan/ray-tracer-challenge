use indicatif::ProgressBar;

use crate::features::{
    canvas::Canvas,
    intersections::{hit, intersect},
    rays::Ray,
    spheres::Sphere,
    tuple::Tuple,
};

pub fn draw_sphere() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_size = 7.0;
    let canvas_pixels = 100.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let shape = Sphere::new();
    let color = Tuple::color(1.0, 0.0, 0.0);
    let bar = ProgressBar::new(canvas.height as u64);
    for y in 0..canvas_pixels as usize {
        let world_y = half - (pixel_size * y as f32);
        for x in 0..canvas_pixels as usize {
            let world_x = -half + (pixel_size * x as f32);
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            let xs = intersect(&shape, &ray);
            if hit(xs).is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
        bar.inc(1);
    }
    canvas.to_ppm();
}
