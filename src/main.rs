mod graphics;

use std::fs::File;
use std::net::TcpStream;
use std::thread;

use clap::Parser;

use graphics::{prelude::*, Animation, Point};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of threads and connections to spawn
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
    /// Number of times to greet
    #[arg(short, long)]
    image: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    thread::scope(|s| {
        for _ in 0..args.threads {
            // let mut img = Image::from_bmp(&args.image).unwrap();
            let gif_file = File::open(&args.image).unwrap();
            let mut gif = Animation::decode_gif(gif_file).unwrap();
            gif.set_position(Point::new(args.x, args.y));
        }
    });
    Ok(())
}

fn loop_stream(to_draw: impl Drawable) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1337").unwrap();

    to_draw.draw_loop(&mut stream)
}
