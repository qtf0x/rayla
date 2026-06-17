// SPDX-License-Identifier: AGPL-3.0-or-later

/* This file is part of the Rayla ray tracer.
 * Copyright (C) 2026 Aster Marias <aster@slware.org>
 *
 * Rayla is free software: you can redistribute it and/or modify it under the terms of the GNU
 * Affero General Public License as published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * Rayla is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with Rayla. If
 * not, see <https://www.gnu.org/licenses/>. */

//! Netpbm graphics formats.
//!
//! Specifications taken from the offical
//! [Netpbm man pages](https://netpbm.sourceforge.net/doc/#formats).

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

fn num_digits(n: u16) -> u8 {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1_000..=9_999 => 4,
        _ => 5,
    }
}

impl ImageEncoder for PpmEncoder {
    fn encode(&self, canvas: &Canvas, writer: &mut impl Write) -> io::Result<()> {
        if canvas.pixels.is_empty() {
            return Err(Error::new(
                ErrorKind::Other,
                "Canvas has no pixels to encode.",
            ));
        }

        let max_channel_val: u16 = match self.color_depth {
            PnmColorDepth::EightBpc => 255,
            PnmColorDepth::SixteenBpc => 65_535,
        };

        // Write image header
        write!(
            writer,
            "P{}\n{} {}\n{}\n",
            match self.encoding {
                PnmEncoding::Ascii => '3',
                PnmEncoding::Binary => '6',
            },
            canvas.width,
            canvas.height,
            max_channel_val
        )?;

        // Write image raster
        let mut raster = canvas
            .pixels
            .iter()
            .flat_map(|color| [color.r, color.g, color.b])
            .map(|sample| (sample.clamp(0.0, 1.0) * max_channel_val as Real).round() as u16);

        match self.encoding {
            PnmEncoding::Ascii => {
                let mut bytes_written = 0;
                let mut line_start = true;

                raster.try_for_each(|sample| -> io::Result<()> {
                    let digits = num_digits(sample) + 1;
                    bytes_written += digits;

                    // Minimize whitespace while limiting lines to 70 bytes
                    if bytes_written > 70 {
                        bytes_written = digits;
                        write!(writer, "\n{sample}")
                    } else if bytes_written == 70 {
                        bytes_written = 0;
                        line_start = true;
                        write!(writer, " {sample}\n")
                    } else if line_start {
                        line_start = false;
                        write!(writer, "{sample}")
                    } else {
                        write!(writer, " {sample}")
                    }
                })
            }

            PnmEncoding::Binary => match self.color_depth {
                PnmColorDepth::EightBpc => raster
                    .try_for_each(|sample| -> io::Result<()> { writer.write_all(&[sample as u8]) }),

                PnmColorDepth::SixteenBpc => raster.try_for_each(|sample| -> io::Result<()> {
                    writer.write_all(&sample.to_be_bytes())
                }),
            },
        }?;

        // Some parsers require a final newline in PPM files
        writer.write_all(b"\n")
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
