use super::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    pub length: f32,
}

impl Ray {
    pub fn new(
        origin: Vector,
        direction: Vector,
        length: f32,
    ) -> Self {
        Self {
            origin,
            direction,
            length,
        }
    }
}
