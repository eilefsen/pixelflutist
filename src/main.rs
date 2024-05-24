mod graphics;

use std::net::TcpStream;
use std::thread;

use graphics::{prelude::*, Image};

fn main() -> std::io::Result<()> {
    thread::scope(|s| {
        s.spawn(loop_stream);
        s.spawn(loop_stream);
        s.spawn(loop_stream);
    });
    Ok(())
}

fn loop_stream() {
    let mut stream = TcpStream::connect("127.0.0.1:1337").unwrap();
    let img = Image::from_bmp("./src/pic.bmp").unwrap();
    loop {
        img.draw(&mut stream).unwrap()
    }
}
