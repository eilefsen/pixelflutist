use std::net::TcpStream;

use graphics::{prelude::*, Image, Pixel};
use graphics::{Point, Rectangle, Rgb};

mod graphics;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1337")?;

    loop {
        Rectangle::new(
            Point::new(0, 0),
            Point::new(240, 320),
            Rgb::new(0xff, 0xff, 0xff),
        )
        .unwrap()
        .draw(&mut stream)?;

        Image::from_bmp("./src/pic.bmp")
            .unwrap()
            .draw(&mut stream)?;
    }
}
