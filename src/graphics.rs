use bmp;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::{fmt, net::TcpStream};

pub mod prelude {
    pub use super::Drawable;
}

mod basics;

pub use basics::*;

#[derive(Debug, Clone)]
pub struct ConflictingPointError;
impl fmt::Display for ConflictingPointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Points conflict, creating a negative Size")
    }
}

pub trait Drawable {
    fn draw(&self, stream: &mut TcpStream) -> std::io::Result<()>;
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
    fn draw(&self, stream: &mut TcpStream) -> std::io::Result<()> {
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

#[derive(Default)]
pub struct Rectangle {
    top_left_corner: Point,
    bottom_right_corner: Point,
    size: Size,
    color: Rgb,
}
impl Rectangle {
    pub fn new(
        top_left_corner: Point,
        bottom_right_corner: Point,
        color: Rgb,
    ) -> Result<Self, ConflictingPointError> {
        if top_left_corner.x > bottom_right_corner.x || top_left_corner.y > bottom_right_corner.y {
            return Err(ConflictingPointError);
        }
        let x = bottom_right_corner.x - top_left_corner.x;
        let y = bottom_right_corner.y - top_left_corner.y;

        let size = Size::new(x, y);

        Ok(Rectangle {
            top_left_corner,
            bottom_right_corner,
            size,
            color,
        })
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
impl Drawable for Rectangle {
    fn draw(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        for x in self.top_left_corner.x..self.bottom_right_corner.x {
            for y in self.top_left_corner.y..self.bottom_right_corner.y {
                Pixel::new(x, y, self.color).draw(stream)?;
            }
        }
        Ok(())
    }
}

pub struct Image {
    img: bmp::Image,
    pub position: Point,
}
impl Image {
    pub fn from_bmp<P>(path: P) -> bmp::BmpResult<Self>
    where
        P: AsRef<Path>,
    {
        let img = bmp::open(path)?;
        Ok(Image {
            img,
            position: Point::default(),
        })
    }
    pub fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}
impl Drawable for Image {
    fn draw(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let height = self.img.get_height();
        let width = self.img.get_width();

        for x in self.position.x..(self.position.x + width) {
            for y in self.position.y..(self.position.y + height) {
                let px = self.img.get_pixel(x, y);
                Pixel::new(x, y, px.into()).draw(stream)?;
            }
        }

        Ok(())
    }
}
