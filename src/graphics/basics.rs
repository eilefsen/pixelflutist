#[derive(Default, Clone, Copy)]
pub struct Size {
    pub x: u32,
    pub y: u32,
}
impl Size {
    pub fn new(x: u32, y: u32) -> Self {
        Size { x, y }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
    pub fn fmt_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl From<bmp::Pixel> for Rgb {
    fn from(px: bmp::Pixel) -> Self {
        Rgb::new(px.r, px.g, px.b)
    }
}
