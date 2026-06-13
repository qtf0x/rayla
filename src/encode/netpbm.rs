/// Netpbm image format encoders
///
use crate::{Canvas, encode::ImageEncoder, math::Real};
use std::io::{self, Error, ErrorKind, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PnmEncoding {
    Ascii,
    Binary,
}

#[derive(Clone, Copy)]
pub enum PnmColorDepth {
    EightBpc,
    SixteenBpc,
}

#[derive(Clone, Copy)]
pub struct PbmEncoder {
    pub encoding: PnmEncoding,
}

#[derive(Clone, Copy)]
pub struct PgmEncoder {
    pub encoding: PnmEncoding,
    pub color_depth: PnmColorDepth,
}

#[derive(Clone, Copy)]
pub struct PpmEncoder {
    pub encoding: PnmEncoding,
    pub color_depth: PnmColorDepth,
}

impl PpmEncoder {
    pub fn plain() -> Self {
        Self {
            encoding: PnmEncoding::Ascii,
            color_depth: PnmColorDepth::EightBpc,
        }
    }

    pub fn raw() -> Self {
        Self {
            encoding: PnmEncoding::Binary,
            color_depth: PnmColorDepth::EightBpc,
        }
    }
}

// TODO: PAM support (https://en.wikipedia.org/wiki/Netpbm#PAM_graphics_format).
// TODO: PFM support (https://en.wikipedia.org/wiki/Netpbm#32-bit).

impl ImageEncoder for PbmEncoder {
    fn encode(&self, canvas: &Canvas, writer: &mut impl Write) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "Not implemented.")) // TODO
    }
}

impl ImageEncoder for PgmEncoder {
    fn encode(&self, canvas: &Canvas, writer: &mut impl Write) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "Not implemented.")) // TODO
    }
}

impl ImageEncoder for PpmEncoder {
    fn encode(&self, canvas: &Canvas, writer: &mut impl Write) -> io::Result<()> {
        let max_channel_val: u16 = match self.color_depth {
            PnmColorDepth::EightBpc => 255,
            PnmColorDepth::SixteenBpc => 65_535,
        };

        let header = format!(
            "P{}\n{} {}\n{}\n",
            match self.encoding {
                PnmEncoding::Ascii => '3',
                PnmEncoding::Binary => '6',
            },
            canvas.width,
            canvas.height,
            max_channel_val
        );
        writer.write_all(header.as_bytes())?;

        for y in 0..canvas.height {
            let mut line_len = 0;

            for x in 0..canvas.width {
                let color = canvas.get(x, y).unwrap();
                let r = (max_channel_val as Real * color.r.clamp(0.0, 1.0)).round() as u16;
                let g = (max_channel_val as Real * color.g.clamp(0.0, 1.0)).round() as u16;
                let b = (max_channel_val as Real * color.b.clamp(0.0, 1.0)).round() as u16;

                let color_str = format!("{r} {g} {b} ");
                let color = match self.encoding {
                    PnmEncoding::Ascii => {
                        let color = color_str.as_bytes();

                        // limit line length to 70 columns
                        let color_len = color.len();
                        if line_len + color_len > 70 {
                            writer.write_all(&[b'\n'])?;
                            line_len = 0;
                        }
                        line_len += color_len;

                        color
                    }
                    PnmEncoding::Binary => match self.color_depth {
                        PnmColorDepth::EightBpc => &[r as u8, g as u8, b as u8] as &[u8],
                        PnmColorDepth::SixteenBpc => &[
                            (r.to_be() >> 0x01) as u8,
                            (r.to_be() & 0x0F) as u8,
                            (g.to_be() >> 0x01) as u8,
                            (g.to_be() & 0x0F) as u8,
                            (b.to_be() >> 0x01) as u8,
                            (b.to_be() & 0x0F) as u8,
                        ] as &[u8],
                    },
                };

                writer.write_all(color)?;
            }

            if self.encoding == PnmEncoding::Ascii {
                writer.write_all(&[b'\n'])?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ColorRGB;

    #[test]
    fn initialize_ppm_header() {
        let (w, h) = (5, 3);
        let canvas = Canvas::new(w, h);

        let mut ppm = Vec::new();
        PpmEncoder::plain().encode(&canvas, &mut ppm).unwrap();

        assert!(
            String::from_utf8(ppm)
                .unwrap()
                .starts_with(&format!("P3\n{w} {h}\n255\n"))
        );
    }

    #[test]
    fn initialize_ppm_body() {
        let (w, h) = (5, 3);
        let mut canvas = Canvas::new(w, h);

        *canvas.get_mut(0, 0).unwrap() = ColorRGB::new(1.5, 0.0, 0.0);
        *canvas.get_mut(2, 1).unwrap() = ColorRGB::new(0.0, 0.5, 0.0);
        *canvas.get_mut(4, 2).unwrap() = ColorRGB::new(-0.5, 0.0, 1.0);

        let mut ppm = Vec::new();
        PpmEncoder::plain().encode(&canvas, &mut ppm).unwrap();

        assert!(
            String::from_utf8(ppm)
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join(" ")
                .ends_with(
                    "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \
                     0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \
                     0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
                )
        );
    }

    #[test]
    fn ppm_split_long_lines() {
        let (w, h) = (10, 2);
        let mut canvas = Canvas::new(w, h);

        canvas.clear(ColorRGB::new(1.0, 0.8, 0.6));
        let mut ppm = Vec::new();
        PpmEncoder::plain().encode(&canvas, &mut ppm).unwrap();

        assert!(
            String::from_utf8(ppm)
                .unwrap()
                .lines()
                .all(|line| line.len() <= 70)
        );
    }

    #[test]
    fn ppm_newline_terminated() {
        let (w, h) = (5, 3);
        let canvas = Canvas::new(w, h);

        let mut ppm = Vec::new();
        PpmEncoder::plain().encode(&canvas, &mut ppm).unwrap();

        assert_eq!(*ppm.last().unwrap(), b'\n');
    }
}
