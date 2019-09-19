use crate::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use nalgebra_glm as glm;
use renderer::RenderState;
use std::cell::RefCell;

pub struct GameState {
    pub render_state: RefCell<RenderState>,
}

impl Default for GameState {
    fn default() -> GameState {
        let projection = glm::perspective(
            DEFAULT_WIDTH as f32 / DEFAULT_HEIGHT as f32,
            45.0,
            0.1,
            100.0,
        );

        let mut view = glm::Mat4::identity();
        view = glm::translate(&view, &glm::vec3(0.0, 0.0, -3.0));

        Self {
            render_state: RefCell::new(RenderState::new(projection, view)),
        }
    }
}
