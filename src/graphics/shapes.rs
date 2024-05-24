use super::{ConflictingPointError, Drawable, Pixel, Point, Rgb, Size};
use std::io::Write;

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
    fn draw(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        for x in self.top_left_corner.x..self.bottom_right_corner.x {
            for y in self.top_left_corner.y..self.bottom_right_corner.y {
                Pixel::new(x, y, self.color).draw(stream)?;
            }
        }
        Ok(())
    }
}
