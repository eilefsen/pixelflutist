mod graphics;

use std::io::Write;
use std::net::TcpStream;
use std::thread;

use graphics::{prelude::*, Image};

fn main() -> std::io::Result<()> {
    thread::scope(|s| {
        s.spawn(loop_stream);
        s.spawn(loop_stream);
        s.spawn(loop_stream);
        s.spawn(loop_stream);
        s.spawn(loop_stream);
        s.spawn(loop_stream);
    });
    Ok(())
}

fn loop_stream() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1337").unwrap();
    let img = Image::from_bmp("./src/pic.bmp").unwrap();
    let mut buf = vec![];
    img.draw(&mut buf).unwrap();
    loop {
        stream.write_all(buf.as_slice())?;
    }
}
