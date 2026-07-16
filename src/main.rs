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

use rayla::math::{self, Matrix4, Point, Real, Vector};
#[cfg(not(feature = "single-precision"))]
use std::f64::consts::TAU;

#[cfg(feature = "single-precision")]
use std::f32::consts::TAU;

fn main() {
    let v = Vector::new(1.0, -2.0, 3.0);
    let p = Point::new(-3.0, 2.0, -1.0);

    println!("Here's a vector: {:?}", v);
    println!("Here's a point:  {:?}", p);

    let twelve = Point::new(0.0, 0.0, 1.0);
    let to_screen = math::translation(20.0, 0.0, 20.0);

    println!("\nClock face (looking down the y axis):");
    for hour in 0..12 {
        let rotate = math::rotation_y(hour as Real * TAU / 12.0);
        let position = (to_screen * rotate) * twelve;

        println!("  {:>2}h -> {:?}", hour, position);
    }

    let spun = Matrix4::IDENTITY
        .rotate_y(TAU / 4.0)
        .translate(5.0, 0.0, 0.0)
        * Point::new(1.0, 0.0, 0.0);

    println!("\nChained rotate+translate of (1, 0, 0): {:?}", spun);
}
