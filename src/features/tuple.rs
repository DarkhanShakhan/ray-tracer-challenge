use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: TupleType,
}
const EPSILON: f32 = 0.00001;

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: TupleType) -> Self {
        Tuple { x, y, z, w }
    }
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Tuple {
            x,
            y,
            z,
            w: TupleType::Point,
        }
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Tuple {
            x,
            y,
            z,
            w: TupleType::Vector,
        }
    }

    pub fn color(x:f32, y:f32, z:f32) -> Self {
        Tuple { x, y, z, w: TupleType::Color}
    }
    pub fn as_str(&self) -> String {
        format!("{} {} {}\n", self.x as i32, self.y as i32, self.z as i32)
    }
    pub fn clamp(&self) -> Self {
        let mut x = self.x *255.0;
        let mut y = self.y* 255.0;
        let mut z = self.z*255.0;
        //TODO:refactor
        if x > 255.0 {
            x= 255.0;
        }
        if x< 0.0 {
            x = 0.0;
        }
        if y > 255.0 {
            y= 255.0;
        }
        if y< 0.0 {
            y = 0.0;
        }
        if z > 255.0 {
            z= 255.0;
        }
        if z< 0.0 {
            z = 0.0;
        }
        Self::color(x, y, z)
    }

    pub fn default_color() ->Self {
        Tuple{x:0.0,y:0.0,z:0.0,w:TupleType::Color}
    }
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.x * self.x).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self::vector(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Tuple) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tuple_tests {
    use super::Tuple;

    #[test]
    fn test_cross() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        let other = Tuple::vector(2.0, 3.0, 4.0);
        let mut res = tuple.cross(&other);
        assert!(res == Tuple::vector(-1.0, 2.0, -1.0));
        res = other.cross(&tuple);
        assert!(res == Tuple::vector(1.0, -2.0, 1.0));
    }
    #[test]
    fn test_mul_colors() {
        let color_tuple= Tuple::color(1.0,  0.2,0.4);
        let color_other_tuple= Tuple::color(0.9,  1.0,0.1);
        let res = color_other_tuple*color_tuple;
        assert!(res == Tuple::color(0.9,0.2,0.04))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x) < EPSILON && (self.y - other.y) < EPSILON && (self.z - other.z) < EPSILON
    }
}
impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, self.w)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w)
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w)
    }
}
impl Mul for Tuple {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::color(self.x *rhs.x, self.y*rhs.y, self.z*rhs.z)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum TupleType {
    Vector,
    #[default]
    Point,
    Color,
    Undefined,
}

impl Add for TupleType {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Point, Self::Vector) | (Self::Vector, Self::Point) => Self::Point,
            (Self::Vector, Self::Vector) => Self::Vector,
            (Self::Color, Self::Color) => Self::Color,
            _ => Self::Undefined,
        }
    }
}
