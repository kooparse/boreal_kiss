use crate::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use engine::{
    camera::Camera, input::Input, platform::PlatformWrapper, time::Time,
};
use nalgebra_glm as glm;
use renderer::RenderState;

pub struct GameState {
    pub camera: Camera,
    pub render_state: RenderState,
}

impl GameState {
    pub fn move_camera(
        &mut self,
        input: &mut Input,
        window: &dyn PlatformWrapper,
        time: &Time,
    ) {
        if input.modifiers.shift {
            window.hide_cursor(true);
            self.render_state.view = self.camera.update(input, time);
        } else {
            window.hide_cursor(false);
        }
    }
}

impl Default for GameState {
    fn default() -> GameState {
        let projection = glm::perspective(
            DEFAULT_WIDTH as f32 / DEFAULT_HEIGHT as f32,
            45.0,
            0.1,
            100.0,
        );

        let camera = Camera::default();
        let view = camera.get_look_at();

        Self {
            render_state: RenderState::new(projection, view),
            camera,
        }
    }
}
