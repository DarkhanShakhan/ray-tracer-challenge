use super::tuple::Tuple;

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Eq for Material {}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod material_tests {
    use crate::features::tuple::Tuple;

    use super::Material;

    #[test]
    fn test_creating_material() {
        let m = Material::new();
        assert_eq!(m.color, Tuple::color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
