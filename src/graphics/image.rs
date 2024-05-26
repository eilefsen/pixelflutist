use png::{ColorType, Transformations};

use super::{Drawable, Pixel, Point, Rgba, Size};

use std::io::{self, Read, Write};
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

    pub fn mirror(&mut self) {
        let data = mirror_rows(self.data.clone(), self.size.x as usize);
        self.data = data;
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
            for v in b.chunks_exact(3) {
                vec.push(Rgba::new(v[0], v[1], v[2], 0xff));
            }
            vec.reverse();
            vec
        };
        Image::new(data, Point::default(), size)
    }

    pub fn from_png<R: Read>(source: R) -> io::Result<Self> {
        let mut decoder = png::Decoder::new(source);
        decoder.set_transformations(Transformations::ALPHA);
        let mut reader = decoder.read_info()?;

        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let size = Size::new(info.width, info.height);
        let mut img = match reader.output_color_type().0 {
            ColorType::Rgba => Ok(Image::from_bytes_rgba(buf.as_slice(), size)),
            ColorType::Rgb => Ok(Image::from_bytes_rgb(buf.as_slice(), size)),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unsupported ColorType: {:?}", reader.output_color_type().0),
            )),
        }?;
        img.mirror();
        Ok(img)
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
    pub fn get_pixel(&self, position: Point) -> io::Result<Rgba> {
        let i = self.get_pixel_location(position);

        self.data.get(i).copied().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid index, {}", i),
        ))
    }
    fn get_pixel_location(&self, position: Point) -> usize {
        ((self.size.y - position.y - 1) * self.size.x + position.x) as usize
    }
}

impl Drawable for Image {
    fn draw(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        for (x, y) in self.coordinates() {
            let px = self.get_pixel(Point::new(x, y))?;
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

fn mirror_rows(data: Vec<Rgba>, width: usize) -> Vec<Rgba> {
    let mut buf = vec![];
    for row in data.clone().chunks_exact_mut(width) {
        row.reverse();
        buf.extend(row.to_vec())
    }
    buf
}
