pub mod netpbm;

use crate::Canvas;
use std::io::{self, Write};

pub trait ImageEncoder {
    fn encode(&self, canvas: &Canvas, writer: &mut impl Write) -> io::Result<()>;
}
