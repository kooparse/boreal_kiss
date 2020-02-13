use crate::tilemap::AbsolutePosition;
use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone)]
pub struct Wall {
    pub position: AbsolutePosition,
    pub float_pos: glm::TVec3<f32>,
    pub is_pushable: bool,
}

impl Wall {
    // We compute the float pos from the AbsolutePosition.
    pub fn new(position: AbsolutePosition, is_pushable: bool) -> Self {
        let float_pos = position.to_float_pos();

        Self {
            position,
            float_pos,
            is_pushable,
        }
    }
}
