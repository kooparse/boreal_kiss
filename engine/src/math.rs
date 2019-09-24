use std::ops::{Add, Sub};

trait RealNumber {}

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x.powf(2.) + (self.y.powf(2.)) + self.z.powf(2.))
    }

    pub fn normalize(&self) -> Vec3 {
        let vector_length = self.length();

        Vec3::new(
            self.x / vector_length,
            self.y / vector_length,
            self.z / vector_length,
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Mat4 {
    inner: [f32; 16],
}

impl Mat4 {
    pub fn indentity() -> Mat4 {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let inner: [f32; 16] = [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ];

        Mat4 { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec3() {
        let a = Vec3::new(1., 2., 3.);
        assert_eq!(a, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn add_vec3() {
        // For float numbers...
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        assert_eq!(a + b, Vec3::new(5., 7., 9.));
    }

    #[test]
    fn sub_vec3() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);

        assert_eq!(a - b, Vec3::new(-3., -3., -3.));
    }

    #[test]
    fn length() {
        let a = Vec3::new(4., 8., 36.);

        assert_eq!(a.length().round(), 37.);
    }

    #[test]
    fn normalize() {
        let a = Vec3::new(4., 8., 36.);

        // The length of a vector normalized is always 1.
        // TODO: I shouldn't have to round this number...
        assert_eq!(a.normalize().length().round(), 1.);
    }

}
