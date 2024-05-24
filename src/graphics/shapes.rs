use super::{ConflictingPointError, Drawable, Pixel, Point, Rgb, Size};
use std::io::Write;

#[derive(Default)]
pub struct Rectangle {
    position: Point,
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
            position: top_left_corner,
            size,
            color,
        })
    }

    pub fn set_position(&mut self, point: Point) {
        self.position = point;
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
impl Drawable for Rectangle {
    fn draw(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        for x in self.position.x..self.size.x {
            for y in self.position.y..self.size.y {
                Pixel::new(x + self.position.x, y + self.position.y, self.color).draw(stream)?;
            }
        }
        Ok(())
    }
}
