use crate::player::MoveAnimation;
use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone)]
pub struct Wall {
    pub world_pos: glm::TVec2<f32>,
    pub map_pos: glm::TVec2<i32>,
    pub is_pushable: bool,

    pub move_animation: Option<MoveAnimation>,
}

impl Wall {
    pub fn new(
        world_pos: glm::TVec2<f32>,
        map_pos: glm::TVec2<i32>,
        is_pushable: bool,
    ) -> Self {
        Self {
            is_pushable,
            world_pos,
            map_pos,
            ..Self::default()
        }
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            world_pos: glm::vec2(0., 0.),
            map_pos: glm::vec2(0, 0),
            is_pushable: false,

            move_animation: None,
        }
    }
}
