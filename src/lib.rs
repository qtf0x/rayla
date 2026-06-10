// Re-exported modules
pub mod math;

use crate::math::ColorRGB;

pub struct Canvas {
    pub width: u16,
    pub height: u16,

    pixels: Vec<ColorRGB>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        // TODO: Ensure that below multiplication will not overflow. This is
        // already guaranteed for 32- and 64-bit systems.
        Self {
            width,
            height,
            pixels: vec![ColorRGB::default(); width as usize * height as usize],
        }
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&ColorRGB> {
        let width = self.width as usize;
        let (x, y) = (x as usize, y as usize);

        self.pixels.get(width * y + x)
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut ColorRGB> {
        let width = self.width as usize;
        let (x, y) = (x as usize, y as usize);

        self.pixels.get_mut(width * y + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_canvas() {
        let (w, h) = (1_920, 1_080);
        let c = Canvas::new(w, h);

        let black = ColorRGB::default();
        assert_eq!(black, ColorRGB::new(0.0, 0.0, 0.0));

        assert_eq!(c.width, w);
        assert_eq!(c.height, h);

        for x in 0..w {
            for y in 0..h {
                assert_eq!(*c.get(x, y).unwrap(), black);
            }
        }

        std::assert_matches!(c.get(w + 1, h + 1), None);
    }

    #[test]
    fn write_pixels_to_canvas() {
        let (w, h) = (1_280, 720);
        let mut c = Canvas::new(w, h);

        let black = ColorRGB::default();
        let red = ColorRGB::new(1.0, 0.0, 0.0);

        *c.get_mut(150, 300).unwrap() = red;

        for x in 0..w {
            for y in 0..h {
                match (x, y) {
                    (150, 300) => assert_eq!(*c.get(x, y).unwrap(), red),
                    _ => assert_eq!(*c.get(x, y).unwrap(), black),
                }
            }
        }
    }
}
