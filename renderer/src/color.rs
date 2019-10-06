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

impl From<[f32; 4]> for Rgba {
    fn from(color_array: [f32; 4]) -> Self {
        Self::new(
            color_array[0], 
            color_array[1], 
            color_array[2],
            color_array[3],
        )
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
#[derive(Debug)]
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

impl From<&[f32; 3]> for Rgb {
    fn from(color_array: &[f32; 3]) -> Self {
        Self::new(
            color_array[0], 
            color_array[1], 
            color_array[2],
        )
    }
}

impl From<&[f32; 4]> for Rgb {
    fn from(color_array: &[f32; 4]) -> Self {
        Self::new(
            color_array[0], 
            color_array[1], 
            color_array[2],
        )
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
