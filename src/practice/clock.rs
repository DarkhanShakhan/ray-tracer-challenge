use std::f32::consts::PI;

use crate::features::{canvas::Canvas, transformations::rotation_y, tuple::Tuple};

pub fn draw_clock() {
    let k = 48.0;
    let mut canvas = Canvas::new(400, 400);
    let canvas_center = (canvas.width / 2, canvas.height / 2);
    let radius = (canvas.width * 3) / 8;
    let r = rotation_y(PI / k);
    let mut twelve = Tuple::point(0.0, 0.0, 1.0);
    let white = Tuple::color(1.0, 1.0, 1.0);
    canvas.write_pixel(canvas_center.0, canvas_center.1, white);
    for _ in 0..(k * 2.0) as usize {
        canvas.write_pixel(
            (twelve.x * radius as f32 + canvas_center.0 as f32) as usize,
            (twelve.z * radius as f32 + canvas_center.1 as f32) as usize,
            white,
        );
        twelve = r.clone() * twelve;
    }
    canvas.to_ppm();
}
