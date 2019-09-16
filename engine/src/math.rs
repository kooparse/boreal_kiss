use std::ops::{Add, Div, Mul, Sub};

// Because we can't alias traits yet.
pub trait GenericNumber:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Clone
    + Copy
    + Sized
{
    // Nothing here... it's an "alias".
}

impl GenericNumber for f64 {}
impl GenericNumber for f32 {}
impl GenericNumber for i32 {}

#[derive(Debug, PartialEq, Eq)]
pub struct Vector3<N: GenericNumber> {
    x: N,
    y: N,
    z: N,
}

impl<N: GenericNumber> Vector3<N> {
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }
}

impl<N: GenericNumber> Add for Vector3<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<N: GenericNumber> Sub for Vector3<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<N: GenericNumber> Mul for Vector3<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<N: GenericNumber> Div for Vector3<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Mat4<N: GenericNumber> {
    inner: [N; 16],
}

impl<N: GenericNumber> Mat4<N> {
    pub fn indentity() -> Mat4<f32> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let inner: [f32; 16] = [
            1., 0., 0., 0., 
            0., 1., 0., 0., 
            0., 0., 1., 0., 
            0., 0., 0., 1.,
        ];

        Mat4::<f32> { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec3() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(4, 5, 6);

        assert_eq!(a + b, Vector3::new(5, 7, 9));
    }

    #[test]
    fn sub_vec3() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(4, 5, 6);

        assert_eq!(a - b, Vector3::new(-3, -3, -3));
    }

    #[test]
    fn mul_vec3() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(4, 5, 6);

        assert_eq!(a * b, Vector3::new(4, 10, 18));
    }

}
