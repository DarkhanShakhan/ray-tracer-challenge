use crate::features::tuple::Tuple;
pub mod clock;
pub mod sphere;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile { position, velocity }
    }
    pub fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}
