use std::ops::{Add, Div, Mul, Sub};

// Because we can't alias traits yet.
pub trait GenericNumber:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sized
{
    // Nothing here... it's an "alias".
}

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
