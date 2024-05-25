use super::{Drawable, Pixel, Point, Size};

use std::io::Write;
use std::path::Path;

#[derive(Clone)]
pub struct Image {
    img: bmp::Image,
    position: Point,
    size: Size,
}
impl Image {
    pub fn from_bmp<P>(path: P) -> bmp::BmpResult<Self>
    where
        P: AsRef<Path>,
    {
        let img = bmp::open(path)?;
        let size = Size::new(img.get_width(), img.get_height());
        Ok(Image {
            img,
            position: Point::default(),
            size,
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
