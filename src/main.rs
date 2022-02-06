mod color;
mod tup;
mod math_helpers;

fn main() {
    println!("Hello World!");
}

use crate::color::Color;

struct Canvas {
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];
        Self {
            width,
            pixels,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.pixels.len() / self.width
    }

    fn pixels(&self) -> Pixels {
        Pixels {
            pixels: self.pixels.iter(),
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn write_pixel(mut self, x: usize, y: usize, c: Color) -> Self {
        let i = self.index(x, y);
        self.pixels[i] = c;
        self
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.index(x, y)]       
    }
}

struct Pixels<'a> {
    pixels: std::slice::Iter<'a, Color>,
}

impl <'a> Iterator for Pixels<'a> {
    type Item = &'a Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.pixels.next()
    }
}

mod canvas_tests {
    use super::*;

    #[test]
    fn a_canvas_stores_its_width() {
        let c = Canvas::new(10, 20);
        assert_eq!(10, c.width());
    }

    #[test]
    fn a_canvas_stored_its_height() {
        let c = Canvas::new(10, 20);
        assert_eq!(20, c.height());
    }

    #[test]
    fn a_new_canvas_has_all_black_pixels() {
        let c = Canvas::new(10, 20);
        let black = Color::new(0.0, 0.0, 0.0);
        assert!(c.pixels().all(|&p| p == black));
    }

    #[test]
    fn a_canvas_pixel_can_be_written_to() {
        let red = Color::new(1.0, 0.0, 0.0);
        let c0 = Canvas::new(10, 20);
        let c1 = c0.write_pixel(2, 3, red);
        let pixel = c1.pixel_at(2, 3);
        assert_eq!(red, pixel);
    }
}

