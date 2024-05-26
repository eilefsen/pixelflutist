mod graphics;

use core::panic;
use std::fs::File;
use std::net::TcpStream;
use std::path::PathBuf;
use std::thread;

use clap::Parser;

use graphics::{prelude::*, Animation, Image, Point};

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Number of threads and connections to spawn
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
    // image to display
    #[arg(short, long)]
    image: PathBuf,
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

enum ImageExtensions {
    Png,
    Bmp,
    Gif,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let extension = match args
        .image
        .extension()
        .unwrap()
        .to_str()
        .expect("filename does not have an extension")
    {
        "png" => ImageExtensions::Png,
        "bmp" => ImageExtensions::Bmp,
        "gif" => ImageExtensions::Gif,
        _ => panic!("Invalid file extension"),
    };

    thread::scope(|s| -> std::io::Result<()> {
        for _ in 0..args.threads {
            let file = File::open(&args.image).unwrap();

            let mut img: Box<dyn Drawable + Send> = match extension {
                ImageExtensions::Png => Box::new(Image::from_png(&file).expect("png error")),
                ImageExtensions::Bmp => Box::new(Image::from_bmp(&args.image).unwrap()),
                ImageExtensions::Gif => Box::new(Animation::decode_gif(&file).unwrap()),
            };
            img.set_position(Point::new(args.x, args.y));
            s.spawn(|| loop_stream(img, args.host.clone(), args.port.clone()).unwrap());
        }
        Ok(())
    })?;
    Ok(())
}

fn loop_stream(to_draw: Box<dyn Drawable>, host: String, port: String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;

    to_draw.draw_loop(&mut stream)
}
