use super::tuple::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Vec<Tuple>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            canvas: vec![vec![Tuple::default_color(); width]; height],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        self.canvas[y][x] = color;
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.canvas[y][x]
    }

    pub fn to_ppm(&self) {
        print!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.canvas.clone().into_iter() {
            for pixel in line {
                print!("{}", pixel.clamp().as_str())
            }
        }
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::features::tuple::Tuple;

    use super::Canvas;

    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::new(10, 20);
        assert!(canvas.width == 10);
        assert!(canvas.height == 20);
    }

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Tuple::color(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        assert!(canvas.pixel_at(2, 3) == red);
    }

    #[test]
    fn test_canvas_to_ppm() {
        let mut canvas = Canvas::new(4, 4);
        let red = Tuple::color(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        canvas.to_ppm();
    }
}
