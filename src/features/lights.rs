use super::tuple::Tuple;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl Light {
    pub fn new(position: Tuple, intensity: Tuple) -> Self {
        Light {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod light_tests {
    use crate::features::tuple::Tuple;

    use super::Light;

    #[test]
    fn test_creating_light() {
        let intensity = Tuple::color(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::new(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }
}
