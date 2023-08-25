use super::{lights::Light, tuple::Tuple};

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

pub fn lightning(
    material: &Material,
    light: &Light,
    position: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
) -> Tuple {
    let effective_color = material.color * light.intensity;
    let lightv = (light.position - *position).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(normalv);
    let mut diffuse = Tuple::default_color();
    let mut specular = Tuple::default_color();

    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    ambient + diffuse + specular
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

#[cfg(test)]
mod lightning_tests {
    use crate::features::{lights::Light, tuple::Tuple};

    use super::{lightning, Material};
    #[test]
    fn test_lightning_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Tuple::color(1.9, 1.9, 1.9))
    }

    #[test]
    fn test_lightning_eye_between_light_and_surface_eye_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt() / 2.0));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Tuple::color(1.0, 1.0, 1.0))
    }

    #[test]
    fn test_lightning_eye_between_light_and_surface_light_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364))
    }

    #[test]
    fn test_lightning_eye_path_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, 10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Tuple::color(0.1, 0.1, 0.1))
    }
}
