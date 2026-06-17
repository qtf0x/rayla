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

//! A ray tracing library and application framework.
//!
//! Rayla provides types and routines for sythesizing images of 3D scenes via the
//! [ray tracing](https://en.wikipedia.org/wiki/Ray_tracing_(graphics)) algorithm. It serves mainly
//! as an educational project; the algorithms, architecture, and techniques used should be assumed
//! neither optimal nor idiomatic.
//!
//! ## References
//!
//! - [*The Ray Tracer Challenge*](http://raytracerchallenge.com/) by Jamis Buck
//! - The [*Ray Tracing in One Weekend*](https://raytracing.github.io/) series by Shirley,
//! Trevor D Black, and Steve Hollasch
//! - [*Real-Time Rendering*](https://www.realtimerendering.com/) by Tomas Akenine-Möller, Eric
//! Haines, Naty Hoffman, Angelo Pesce, Michał Iwanicki, and Sébastien Hillaire
//!
//! <img src= "https://upload.wikimedia.org/wikipedia/commons/0/06/AGPLv3_Logo.svg"
//! alt="GNU AGPLv3 Logo" title="GNU Affero General Public License" width="150"/>

// Exported modules
pub mod encode;
pub mod math;

use crate::math::ColorRGB;

/// High-precision internal storage/back buffer for pixel write operations.
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

    pub fn write(&mut self, x: u16, y: u16, color: ColorRGB) {
        let Some(pixel) = self.get_mut(x, y) else {
            return; // does nothing if index out of bounds
        };

        *pixel = color;
    }

    pub fn clear(&mut self, color: ColorRGB) {
        self.pixels.fill(color);
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

        for (x, y) in (0..w).zip(0..h) {
            assert_eq!(*c.get(x, y).unwrap(), black);
        }

        std::assert_matches!(c.get(w + 1, h + 1), None);
    }

    #[test]
    #[ignore]
    fn big_canvas() {
        let (w, h) = (30_720, 17_280); // 32k resolution
        let c = Canvas::new(w, h);

        std::assert_matches!(c.get(w - 1, h - 1), Some(_));
        std::assert_matches!(c.get(w, h), None);
    }

    #[test]
    fn write_pixels_to_canvas() {
        let (w, h) = (1_280, 720);
        let mut c = Canvas::new(w, h);

        let black = ColorRGB::default();
        let red = ColorRGB::new(1.0, 0.0, 0.0);

        c.write(150, 300, red);

        for (x, y) in (0..w).zip(0..h) {
            match (x, y) {
                (150, 300) => assert_eq!(*c.get(x, y).unwrap(), red),
                _ => assert_eq!(*c.get(x, y).unwrap(), black),
            }
        }
    }
}
