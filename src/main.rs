use features::tuple::Tuple;
use practice::{Environment, Projectile};

mod features;
mod practice;
fn main() {
    let mut proj = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.0, 0.0).normalize(),
    );
    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    loop {
        println!("Projectile position: {:?}", proj.position);
        proj.tick(&env);
        if proj.position.y <= 0.0 {
            break;
        }
    }
}
