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
