// SPDX-License-Identifier: AGPL-3.0-or-later

/* This file is part of the Rayla ray tracer.
 * Copyright (C) 2026 Aster Marias <aster@slware.org>
 *
 * Rayla is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rayla is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Rayla.  If not, see <https://www.gnu.org/licenses/>. */

//! Image serialization facilities and supported formats.

// Exported submodules
pub mod netpbm;

use crate::Canvas;
use std::io::{self, Write};

/// Types that knows how to encode pixel data in some storage format.
///
/// Implementors ("encoders") are usually structs storing format parameters (e.g., color depth,
/// compression ratio).
pub trait ImageEncoder {
    fn encode(&self, canvas: &Canvas, writer: &mut impl Write) -> io::Result<()>;
}
