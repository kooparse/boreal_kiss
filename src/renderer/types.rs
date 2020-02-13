use nalgebra_glm as glm;
use std::cmp::PartialEq;
use std::ops::{Add, Mul};

#[derive(Debug, Default, Copy, Clone)]
pub struct Dimension {
    pub width: f32,
    pub height: f32,
}

impl Dimension {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn get(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn center(&self) -> (f32, f32) {
        (self.width * 0.5, self.height * 0.5)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn get(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn to_glm(&self) -> glm::TVec2<f32> {
        glm::vec2(self.x, self.y)
    }
}

//
//
// COLOR
//
//
pub trait Colors {
    fn red() -> Self;
    fn green() -> Self;
    fn blue() -> Self;
    fn black() -> Self;
    fn white() -> Self;
}
/// Define RGBA color.
/// From 0 (black) to 1 (white).
#[derive(Debug, Copy, Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Colors for Rgba {
    fn red() -> Self {
        Self::new(1., 0., 0., 1.)
    }
    fn black() -> Self {
        Self::new(0., 0., 0., 1.)
    }
    fn white() -> Self {
        Self::new(1., 1., 1., 1.)
    }
    fn green() -> Self {
        Self::new(0., 1., 0., 1.)
    }
    fn blue() -> Self {
        Self::new(0., 0., 1., 1.)
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

impl From<Rgba> for [f32; 3] {
    fn from(rgba: Rgba) -> Self {
        [rgba.r, rgba.g, rgba.b]
    }
}

impl From<Rgba> for [f32; 4] {
    fn from(rgba: Rgba) -> Self {
        [rgba.r, rgba.g, rgba.b, rgba.a]
    }
}

impl From<&Rgb> for Rgba {
    fn from(rgb: &Rgb) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, 1.)
    }
}

/// Define RGBA color.
/// From 0 (black) to 1 (white).
#[derive(Debug, Copy, Clone)]
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

impl Colors for Rgb {
    fn red() -> Self {
        Self::new(1., 0., 0.)
    }
    fn black() -> Self {
        Self::new(0., 0., 0.)
    }
    fn white() -> Self {
        Self::new(1., 1., 1.)
    }
    fn green() -> Self {
        Self::new(0., 1., 0.)
    }
    fn blue() -> Self {
        Self::new(0., 0., 1.)
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
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
        Self::new(rgba.r, rgba.g, rgba.b)
    }
}

//
//
// Position/Vectors...
//
//

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub f32, pub f32, pub f32);

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Add<f32> for Vector {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Vector {
    pub fn to_glm(&self) -> glm::TVec3<f32> {
        glm::vec3(self.0, self.1, self.2)
    }

    pub fn from_glm(vec_glm: glm::TVec3<f32>) -> Self {
        Self(vec_glm.x, vec_glm.y, vec_glm.z)
    }
}

impl From<&Vector> for [f32; 3] {
    fn from(vector: &Vector) -> Self {
        [vector.0, vector.1, vector.2]
    }
}

impl From<&Vector> for [f32; 2] {
    fn from(vector: &Vector) -> Self {
        [vector.0, vector.1]
    }
}
