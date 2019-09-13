use std::ops::{Add, Sub};

pub struct Vector3<N: Add> {
    x: N,
    y: N,
    z: N,
}

impl<N: Add> Vector3<N> {
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }
}

impl<N: Add<Output = N>> Add for Vector3<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<N: Add + Sub<Output = N>> Sub for Vector3<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
