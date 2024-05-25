mod gif;
mod image;
mod primitives;
pub mod shapes;

pub use gif::*;
pub use image::*;
pub use primitives::*;
pub use shapes::Pixel;

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
    // Draws Pixels to writer
    fn draw(&self, writer: &mut dyn Write) -> std::io::Result<()>;
    //
    fn draw_loop(&self, writer: &mut dyn Write) -> std::io::Result<()>;
}
