use std::cmp::PartialEq;

/// Define RGBA color.
/// From 0 (black) to 1 (white).
#[derive(Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl PartialEq for Rgba {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r 
            && self.g == other.g 
            && self.b == other.b 
            && self.a == other.a
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::new(1., 1., 1., 1.)
    }
}

impl From<&Rgba> for [f32; 3] {
    fn from(rgba: &Rgba) -> Self {
        [rgba.r, rgba.g, rgba.b]
    }
}

impl From<&Rgba> for [f32; 4] {
    fn from(rgba: &Rgba) -> Self {
        [rgba.r, rgba.g, rgba.b, rgba.a]
    }
}

impl From<&Rgb> for Rgba {
    fn from(rgb: &Rgb) -> Self {
        Self::new(
            rgb.r, 
            rgb.g, 
            rgb.b,
            1.,
        )
    }
}

/// Define RGBA color.
/// From 0 (black) to 1 (white).
#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r 
            && self.g == other.g 
            && self.b == other.b 
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::new(1., 1., 1.)
    }
}

impl From<&Rgb> for [f32; 3] {
    fn from(rgb: &Rgb) -> Self {
        [rgb.r, rgb.g, rgb.b]
    }
}

impl From<&Rgb> for [f32; 4] {
    fn from(rgb: &Rgb) -> Self {
        [rgb.r, rgb.g, rgb.b, 1.]
    }
}


impl From<&Rgba> for Rgb {
    fn from(rgba: &Rgba) -> Self {
        Self::new(
            rgba.r, 
            rgba.g, 
            rgba.b,
        )
    }
}
