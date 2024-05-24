mod graphics;

use std::io::Write;
use std::net::TcpStream;
use std::thread;

use clap::Parser;

use graphics::{prelude::*, Image};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    thread::scope(|s| {
        for _ in 0..args.threads {
            s.spawn(loop_stream);
        }
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
