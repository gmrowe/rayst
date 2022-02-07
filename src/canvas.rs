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

    fn pixels_mut(&mut self) -> PixelsMut {
        PixelsMut {
            pixels: self.pixels.iter_mut(),
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

    fn break_line(s: &str, max_len: usize) -> String {
        if s.chars().count() < max_len {
            s.to_string()
        } else {
            let break_index = s.rmatch_indices(' ')
                .find(|(i, _)| *i < max_len)
                .map(|(i, _)| i)
                .unwrap_or(max_len - 1);
            let begin = &s[..break_index];
            let end = Self::break_line(&s[break_index + 1..], max_len);
            format!("{}\n{}", begin, end)
        }
    }

    fn to_ppm(&self) -> String {
        const MAX_LINE_LEN: usize = 70;
        let header = format!("P3\n{} {}\n255", self.width(), self.height());
        let mut ppm_lines = Vec::new();
        for line in self.pixels.chunks(self.width()) {
            let mut triples = Vec::new();
            for pixel in line.iter() {
                let (r, g, b) = pixel.to_byte_triple();
                triples.push(format!("{} {} {}", r, g, b));
            }
            let pixel_line = triples.join(" ");
            let truncated = Self::break_line(&pixel_line, MAX_LINE_LEN);
            ppm_lines.push(truncated);
        }
        let body = ppm_lines.join("\n");
        format!("{}\n{}\n", header, body)
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

struct PixelsMut<'a> {
    pixels: std::slice::IterMut<'a, Color>,
}

impl <'a> Iterator for PixelsMut<'a> {
    type Item = &'a mut Color;

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
        let mut c = Canvas::new(10, 20);
        c = c.write_pixel(2, 3, red);
        assert_eq!(red, c.pixel_at(2, 3));
    }

    #[test]
    fn a_canvas_can_construct_a_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let first_3_lines: Vec<&str> = ppm.lines().take(3).collect();
        let expected = vec!["P3", "5 3", "255"];
        assert_eq!(expected, first_3_lines);
    }

    #[test]
    fn a_canvas_can_output_ppm_data() {
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        let c = Canvas::new(5, 3)
            .write_pixel(0, 0, c1)
            .write_pixel(2, 1, c2)
            .write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let lines_4_to_6: Vec<&str> = ppm.lines()
            .skip(3)
            .take(3)
            .collect();
        let expected = vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        ];
        assert_eq!(expected, lines_4_to_6);
        
    }

    #[test]
    fn long_lines_in_ppm_files_should_be_split() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for pixel in c.pixels_mut() {
            *pixel = color;
        }
        let ppm = c.to_ppm();
        let lines_4_to_7: Vec<&str> = ppm.lines()
            .skip(3)
            .take(4)
            .collect();
        let expected = vec![
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        ];
        assert_eq!(expected, lines_4_to_7);
    }

    #[test]
    fn ppm_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let last = ppm.chars().last().expect("This call should never fail");
        assert_eq!('\n', last);
    }
}
