use super::{Vector, Rgba};

pub enum LightSource {
    Sun(SunLight),
}

pub struct SunLight {
    position: Vector,
    direction: Vector,
    ambient: Rgba,
}

impl SunLight {
    pub fn new(position: Vector, direction: Vector, ambient: Rgba) -> Self {
        Self {
            position,
            direction,
            ambient,
        }
    }
}


