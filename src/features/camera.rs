pub mod cameras {
    use indicatif::ProgressBar;

    use crate::features::{
        canvas::Canvas, matrice::Matrice, rays::Ray, tuple::Tuple, world::World,
    };

    pub struct Camera {
        pub hsize: f32,
        pub vsize: f32,
        pub field_of_view: f32,
        pub transform: Matrice,
        pub pixel_size: f32,
        pub half_width: f32,
        pub half_height: f32,
    }

    impl Camera {
        pub fn new(hsize: f32, vsize: f32, field_of_view: f32) -> Camera {
            let mut out = Self {
                hsize,
                vsize,
                field_of_view,
                transform: Matrice::identity_matrix(4),
                pixel_size: 0.0,
                half_height: 0.0,
                half_width: 0.0,
            };
            let half_view = (field_of_view / 2.0).tan();
            let aspect = hsize / vsize;
            if aspect >= 1.0 {
                out.half_width = half_view;
                out.half_height = half_view / aspect;
            } else {
                out.half_width = half_view * aspect;
                out.half_height = half_view;
            }
            out.pixel_size = (out.half_width * 2.0) / out.hsize;
            out
        }
        pub fn ray_for_pixel(&self, px: f32, py: f32) -> Ray {
            // the offset from the edge of the canvas to the pixel's center
            let xoffset = (px + 0.5) * self.pixel_size;
            let yoffset = (py + 0.5) * self.pixel_size;

            //  the untransformed coordinates of the pixel in world space.
            // (remember that the camera looks toward -z, so +x is to the *left*.)
            let world_x = self.half_width - xoffset;
            let world_y = self.half_height - yoffset;

            //  using the camera matrix, transform the canvas point and the origin,
            //  and then compute the ray's direction vector.
            //  (remember that the canvas is at z=-1)

            let pixel = self.transform.inverse().unwrap() * Tuple::point(world_x, world_y, -1.0);
            let origin = self.transform.inverse().unwrap() * Tuple::point(0.0, 0.0, 0.0);
            let direction = (pixel - origin).normalize();
            Ray::new(origin, direction)
        }
        pub fn render(&self, world: &World) -> Canvas {
            let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
            let bar = ProgressBar::new(self.vsize as u64);
            for y in 0..self.vsize as usize - 1 {
                for x in 0..self.hsize as usize - 1 {
                    let ray = self.ray_for_pixel(x as f32, y as f32);
                    let color = world.color_at(&ray);
                    image.write_pixel(x, y, color);
                }
                bar.inc(1);
            }
            image
        }
    }

    #[cfg(test)]
    mod cameras_tests {
        use std::f32::consts::PI;

        use super::*;
        #[test]
        fn testing_camera_construction() {
            let hsize = 160.0;
            let vsize = 120.0;
            let field_of_view = PI / 2.0;
            let c = Camera::new(hsize, vsize, field_of_view);
            assert_eq!(c.hsize, hsize);
            assert_eq!(c.vsize, vsize);
            assert_eq!(c.field_of_view, field_of_view);
            assert_eq!(c.transform, Matrice::identity_matrix(4));
        }
        #[test]
        fn testing_pixel_size_horizontal_canvas() {
            let c = Camera::new(200.0, 125.0, PI / 2.0);
            assert_eq!(c.pixel_size, 0.01);
        }

        #[test]
        fn testing_pixel_size_vertical_canvas() {
            let c = Camera::new(125.0, 200.0, PI / 2.0);
            assert_eq!(c.pixel_size, 0.01);
        }
    }

    #[cfg(test)]
    mod ray_for_pixel_tests {
        use std::f32::consts::PI;

        use crate::features::transformations::{rotation_y, translation};

        use super::*;
        #[test]
        fn test_ray_through_canvas_center() {
            let c = Camera::new(201.0, 101.0, PI / 2.0);
            let r = c.ray_for_pixel(100.0, 50.0);
            assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
            assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
        }

        #[test]
        fn test_ray_through_canvas_corner() {
            let c = Camera::new(201.0, 101.0, PI / 2.0);
            let r = c.ray_for_pixel(0.0, 0.0);
            assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
            assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
        }

        #[test]
        fn test_ray_camera_transformed() {
            let mut c = Camera::new(201.0, 101.0, PI / 2.0);
            c.transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
            let r = c.ray_for_pixel(100.0, 50.0);
            assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
            assert_eq!(
                r.direction,
                Tuple::vector(2.0_f32.sqrt() / 2.0, 0.0, -(2.0_f32.sqrt() / 2.0))
            );
        }
    }
    #[cfg(test)]
    mod rendering_tests {
        use std::f32::consts::PI;

        use crate::features::transformations::view_transformation;

        use super::*;
        #[test]
        fn test_rendering_world_with_camera() {
            let world = World::default();
            let mut c = Camera::new(11.0, 11.0, PI / 2.0);
            let from = Tuple::point(0.0, 0.0, -5.0);
            let to = Tuple::point(0.0, 0.0, 0.0);
            let up = Tuple::vector(0.0, 1.0, 0.0);
            c.transform = view_transformation(from, to, up);
            let image = c.render(&world);
            assert_eq!(image.pixel_at(5, 5), Tuple::color(0.38066, 0.47583, 0.2855))
        }
    }
}
