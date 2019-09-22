use nalgebra_glm as glm;

pub struct Ray {
    pub origin: glm::TVec3<f32>,
    pub direction: glm::TVec3<f32>,
    pub length: f32,
}

impl Ray {
    pub fn new(
        origin: glm::TVec3<f32>,
        direction: glm::TVec3<f32>,
        length: f32,
    ) -> Self {
        Self {
            origin,
            direction,
            length,
        }
    }
}
