use features::{canvas::Canvas, tuple::Tuple, matrice::Matrice};
use practice::{Environment, Projectile};

mod features;
mod practice;
fn main() {
   


    let mut proj = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 12.6,
    );
    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let red = Tuple::color(1.0, 0.0, 0.0);
    let mut canvas = Canvas::new(900, 550);
    loop {
        if (1 + proj.position.y as usize) > canvas.height || proj.position.y <= 0.0 || proj.position.x as usize > canvas.width {
            break;
        }
        canvas.write_pixel(proj.position.x as usize, canvas.height -1- proj.position.y as usize, red);
        proj.tick(&env);
    }
    canvas.to_ppm();
}
