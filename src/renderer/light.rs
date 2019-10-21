use super::{Vector, Rgba};

#[derive(Debug)]
pub enum LightProbes {
    Sun(SunLight),
}

#[derive(Debug)]
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


