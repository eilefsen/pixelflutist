use std::io::{Read, Write};

use crate::Drawable;

use super::{Image, Point, Size};

#[derive(Clone)]
pub struct Animation {
    frames: Vec<Frame>,
    delay_hundreths: u16,
}
impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        let delay_hundreths = frames[0].delay_hundreths;

        Animation {
            frames,
            delay_hundreths,
        }
    }

    pub fn delay_hundreths(&self) -> u16 {
        self.delay_hundreths
    }

    pub fn from_images(images: Vec<Image>, delay_hundreths: u16) -> Self {
        let frames = images
            .iter()
            .map(|x| Frame::new(x.clone(), delay_hundreths))
            .collect::<Vec<Frame>>();

        Animation {
            frames,
            delay_hundreths,
        }
    }

    pub fn decode_gif(reader: impl Read) -> Self {
        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let mut decoder = decoder.read_info(reader).unwrap();

        let mut frames = vec![];

        while let Some(frame) = decoder.read_next_frame().unwrap() {
            let size = Size::new(frame.width.into(), frame.height.into());

            let mut image = match frame.transparent {
                Some(_) => Image::from_bytes_rgba(&frame.buffer, size),
                None => Image::from_bytes_rgb(&frame.buffer, size),
            };

            image.set_position(Point::new(0, 0));

            let f = Frame::new(image, frame.delay);

            frames.push(f);
        }

        Animation::new(frames)
    }
}

impl Drawable for Animation {
    fn draw(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        for f in self.frames.as_slice() {
            f.image.draw(stream)?;
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

impl From<Vec<Image>> for Animation {
    fn from(images: Vec<Image>) -> Self {
        let frames = images
            .iter()
            .map(|x| Frame::new(x.clone(), 2))
            .collect::<Vec<Frame>>();

        Animation::new(frames)
    }
}

#[derive(Clone)]
pub struct Frame {
    image: Image,
    delay_hundreths: u16,
}

impl Frame {
    pub fn new(image: Image, delay_hundreths: u16) -> Self {
        Frame {
            image,
            delay_hundreths,
        }
    }
}
