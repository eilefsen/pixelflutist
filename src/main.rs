mod graphics;

use std::fs::File;
use std::net::TcpStream;
use std::thread;

use clap::Parser;

use graphics::{prelude::*, Animation, Point};

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Number of threads and connections to spawn
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
    // image to display
    #[arg(short, long)]
    image: String,
    // position x
    #[arg(short, default_value_t = 0)]
    x: u32,
    // position y
    #[arg(short, default_value_t = 0)]
    y: u32,

    // Host of pixelflut server
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,
    // Port of pixelflut server
    #[arg(long, default_value_t = String::from("1337"))]
    port: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    thread::scope(|s| {
        for _ in 0..args.threads {
            // let mut img = Image::from_bmp(&args.image).unwrap();
            let gif_file = File::open(&args.image).unwrap();
            let mut gif = Animation::decode_gif(gif_file).unwrap();
            gif.set_position(Point::new(args.x, args.y));
            s.spawn(|| loop_stream(gif, args.host.clone(), args.port.clone()));
        }
    });
    Ok(())
}

fn loop_stream(to_draw: impl Drawable, host: String, port: String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

    to_draw.draw_loop(&mut stream)
}
