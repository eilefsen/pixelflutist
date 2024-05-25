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
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Rgba { r, g, b, a }
    }
    pub fn fmt_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}

impl From<bmp::Pixel> for Rgba {
    fn from(px: bmp::Pixel) -> Self {
        Rgba::new(px.r, px.g, px.b, 0xff)
    }
}
