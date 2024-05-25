use super::{ConflictingPointError, Drawable, Point, Rgba, Size};
use std::io::Write;

pub trait Shape {
    fn set_position(&mut self, point: Point);
    fn set_color(&mut self, color: Rgba);
}

#[derive(Default)]
pub struct Rectangle {
    position: Point,
    size: Size,
    color: Rgba,
}
impl Rectangle {
    pub fn new(
        top_left_corner: Point,
        bottom_right_corner: Point,
        color: Rgba,
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
    fn draw_loop(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let mut buf = vec![];
        self.draw(&mut buf)?;

        loop {
            writer.write_all(buf.as_slice())?;
        }
    }
}
impl Shape for Rectangle {
    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
    fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }
}

#[derive(Default, Clone, Copy)]
pub struct Pixel {
    pub position: Point,
    pub color: Rgba,
}
impl Pixel {
    pub fn new(x: u32, y: u32, color: Rgba) -> Self {
        Pixel {
            position: Point { x, y },
            color,
        }
    }
}
impl Shape for Pixel {
    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
    fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }
}
impl Drawable for Pixel {
    fn draw(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        let s = format!(
            "PX {} {} {}\n",
            self.position.x,
            self.position.y,
            self.color.fmt_hex()
        );
        let _ = stream.write(s.as_bytes())?;

        Ok(())
    }
    fn draw_loop(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let mut buf = vec![];
        self.draw(&mut buf)?;

        loop {
            writer.write_all(buf.as_slice())?;
        }
    }
}
