mod image;
mod primitives;
pub mod shapes;

pub use image::*;
pub use primitives::*;

use std::fmt;
use std::io::prelude::*;

pub mod prelude {
    pub use super::Drawable;
}

#[derive(Debug, Clone)]
pub struct ConflictingPointError;
impl fmt::Display for ConflictingPointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Points conflict, creating a negative Size")
    }
}

pub trait Drawable {
    fn draw(&self, stream: &mut dyn Write) -> std::io::Result<()>;
}

#[derive(Default)]
pub struct Pixel {
    pub point: Point,
    pub color: Rgb,
}
impl Pixel {
    pub fn new(x: u32, y: u32, color: Rgb) -> Self {
        Pixel {
            point: Point { x, y },
            color,
        }
    }
}
impl Drawable for Pixel {
    fn draw(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        let s = format!(
            "PX {} {} {}\n",
            self.point.x,
            self.point.y,
            self.color.fmt_hex()
        );
        let _ = stream.write(s.as_bytes())?;

        Ok(())
    }
}
