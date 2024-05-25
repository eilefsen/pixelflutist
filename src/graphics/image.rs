use super::{Drawable, Pixel, Point, Rgba, Size};

use std::io::Write;
use std::path::Path;

#[derive(Clone)]
pub struct Image {
    data: Vec<Rgba>,
    position: Point,
    size: Size,
}
impl Image {
    pub fn new(data: Vec<Rgba>, position: Point, size: Size) -> Self {
        Image {
            data,
            position,
            size,
        }
    }

    pub fn from_bytes_rgba(b: &[u8], size: Size) -> Self {
        let data = {
            let mut vec: Vec<Rgba> = Vec::new();
            vec.try_reserve_exact(b.len() + size.x as usize * size.y as usize)
                .expect("OOM");
            for v in b.chunks_exact(4) {
                vec.push(Rgba::new(v[0], v[1], v[2], v[3]));
            }
            vec.reverse();
            vec
        };
        Image::new(data, Point::default(), size)
    }
    pub fn from_bytes_rgb(b: &[u8], size: Size) -> Self {
        let data = {
            let mut vec: Vec<Rgba> = Vec::new();
            vec.try_reserve_exact(b.len() + size.x as usize * size.y as usize)
                .expect("OOM");
            for v in b.chunks_exact(4) {
                vec.push(Rgba::new(v[0], v[1], v[2], 0xff));
            }
            vec.reverse();
            vec
        };
        Image::new(data, Point::default(), size)
    }

    pub fn from_bmp<P>(path: P) -> bmp::BmpResult<Self>
    where
        P: AsRef<Path>,
    {
        let img = bmp::open(path)?;
        let size = Size::new(img.get_width(), img.get_height());
        let position = Point::default();

        let mut data: Vec<Rgba> = Vec::with_capacity((size.x * size.y) as usize);

        for (x, y) in img.coordinates() {
            let px = img.get_pixel(x, y);
            data.push(Rgba::from(px));
        }

        Ok(Image {
            position,
            size,
            data,
        })
    }

    // Shamelessly ripping off bmp::Image struct implementation
    // rust-bmp
    // Licensed under MIT license
    // Copyright (c) 2015 Sondre Lefsaker
    pub fn coordinates(&self) -> ImageIndex {
        ImageIndex::new(self.size)
    }
    pub fn set_pixel(&mut self, position: Point, px: Rgba) {
        let mut _x = self.data[self.get_pixel_location(position)];
        _x = px;
    }
    pub fn get_pixel(&self, position: Point) -> Rgba {
        self.data[self.get_pixel_location(position)]
    }
    fn get_pixel_location(&self, position: Point) -> usize {
        ((self.size.y - position.y - 1) * self.size.x + position.x) as usize
    }
}

impl Drawable for Image {
    fn draw(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        for (x, y) in self.coordinates() {
            let px = self.get_pixel(Point::new(x, y));
            Pixel::new(x + self.position.x, y + self.position.y, px).draw(writer)?;
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
    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}

// Another shameless rip off
// rust-bmp
// Licensed under MIT license
// Copyright (c) 2015 Sondre Lefsaker
#[derive(Clone, Copy)]
pub struct ImageIndex {
    size: Size,
    point: Point,
}

impl ImageIndex {
    fn new(size: Size) -> ImageIndex {
        ImageIndex {
            size,
            point: Point::default(),
        }
    }
}

impl Iterator for ImageIndex {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.point.x < self.size.x && self.point.y < self.size.y {
            let this = Some((self.point.x, self.point.y));
            self.point.x += 1;
            if self.point.x == self.size.x {
                self.point.x = 0;
                self.point.y += 1;
            }
            this
        } else {
            None
        }
    }
}
