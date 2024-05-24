use super::{Drawable, Pixel, Point};

use std::io::Write;
use std::path::Path;

#[derive(Clone)]
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
    fn draw(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let height = self.img.get_height();
        let width = self.img.get_width();

        for x in 0..width {
            for y in 0..height {
                let px = self.img.get_pixel(x, y);
                Pixel::new(x + self.position.x, y + self.position.y, px.into()).draw(writer)?;
            }
        }

        Ok(())
    }
}
