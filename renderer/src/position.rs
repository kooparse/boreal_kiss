use std::ops::Mul;
use nalgebra_glm as glm;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub f32, pub f32, pub f32);

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)

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
