use super::{lights::Light, patterns::Pattern, shape::Shape, tuple::Tuple};

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub pattern: Option<Pattern>,
    pub color: Tuple,
    pub ambient: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
}

impl Eq for Material {}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0),
            pattern: None,
            transparency: 0.0,
            refractive_index: 1.0,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }
}

pub fn lightning(
    material: &Material,
    shape: &Shape,
    light: &Light,
    position: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
) -> Tuple {
    let mut color = material.color;
    if let Some(c) = &material.pattern {
        color = c.at_object(shape, position);
    }
    let effective_color = color * light.intensity;
    let lightv = (light.position - *position).normalize();
    let ambient = effective_color * material.ambient;
    if in_shadow {
        return ambient;
    }
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
mod reflective_tests {
    use super::*;

    #[test]
    fn test_reflective_field() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }
}

#[cfg(test)]
mod transparency_refractive_tests {
    use super::*;

    #[test]
    fn test_transparency_refractive_index_default_material() {
        let m = Material::new();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}

// #[cfg(test)]
// mod lightning_tests {
//     use crate::features::{lights::Light, patterns::Pattern, tuple::Tuple};

//     use super::{lightning, Material};
//     #[test]
//     fn test_lightning_eye_between_light_and_surface() {
//         let m = Material::new();
//         let position = Tuple::point(0.0, 0.0, 0.0);
//         let eyev = Tuple::vector(0.0, 0.0, -1.0);
//         let normalv = Tuple::vector(0.0, 0.0, -1.0);
//         let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
//         let result = lightning(&m, &light, &position, &eyev, &normalv, false);
//         assert_eq!(result, Tuple::color(1.9, 1.9, 1.9))
//     }

//     #[test]
//     fn test_lightning_eye_between_light_and_surface_eye_45() {
//         let m = Material::new();
//         let position = Tuple::point(0.0, 0.0, 0.0);
//         let eyev = Tuple::vector(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt() / 2.0));
//         let normalv = Tuple::vector(0.0, 0.0, -1.0);
//         let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
//         let result = lightning(&m, &light, &position, &eyev, &normalv, false);
//         assert_eq!(result, Tuple::color(1.0, 1.0, 1.0))
//     }

//     #[test]
//     fn test_lightning_eye_between_light_and_surface_light_45() {
//         let m = Material::new();
//         let position = Tuple::point(0.0, 0.0, 0.0);
//         let eyev = Tuple::vector(0.0, 0.0, -1.0);
//         let normalv = Tuple::vector(0.0, 0.0, -1.0);
//         let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
//         let result = lightning(&m, &light, &position, &eyev, &normalv, false);
//         assert_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364))
//     }

//     #[test]
//     fn test_lightning_eye_path_reflection_vector() {
//         let m = Material::new();
//         let position = Tuple::point(0.0, 0.0, 0.0);
//         let eyev = Tuple::vector(0.0, 0.0, -1.0);
//         let normalv = Tuple::vector(0.0, 0.0, -1.0);
//         let light = Light::new(Tuple::point(0.0, 0.0, 10.0), Tuple::color(1.0, 1.0, 1.0));
//         let result = lightning(&m, &light, &position, &eyev, &normalv, false);
//         assert_eq!(result, Tuple::color(0.1, 0.1, 0.1))
//     }

//     #[test]
//     fn test_lightning_in_shadow() {
//         let m = Material::new();
//         let position = Tuple::point(0.0, 0.0, 0.0);
//         let eyev = Tuple::vector(0.0, 0.0, -1.0);
//         let normalv = Tuple::vector(0.0, 0.0, -1.0);
//         let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
//         let in_shadow = true;
//         let result = lightning(&m, &light, &position, &eyev, &normalv, in_shadow);
//         assert_eq!(result, Tuple::color(0.1, 0.1, 0.1))
//     }

//     #[test]
//     fn lightning_with_pattern_applied() {
//         let mut m = Material::new();
//         m.pattern = Some(Pattern::new(
//             Tuple::color(1.0, 1.0, 1.0),
//             Tuple::color(0.0, 0.0, 0.0),
//         ));
//         m.ambient = 1.0;
//         m.diffuse = 0.0;
//         m.specular = 0.0;
//         let eyev = Tuple::vector(0.0, 0.0, -1.0);
//         let normalv = Tuple::vector(0.0, 0.0, -1.0);
//         let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
//         assert_eq!(
//             lightning(
//                 &m,
//                 &light,
//                 &Tuple::point(0.9, 0.0, 0.0),
//                 &eyev,
//                 &normalv,
//                 false
//             ),
//             Tuple::color(1.0, 1.0, 1.0)
//         );
//         assert_eq!(
//             lightning(
//                 &m,
//                 &light,
//                 &Tuple::point(1.1, 0.0, 0.0),
//                 &eyev,
//                 &normalv,
//                 false
//             ),
//             Tuple::color(0.0, 0.0, 0.0)
//         );
//     }
// }
