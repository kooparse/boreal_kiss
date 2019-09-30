use crate::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use engine::camera::Camera;
use nalgebra_glm as glm;
use renderer::{GameResolution, RenderState};

pub struct GameState {
    pub camera: Camera,
    pub render_state: RenderState,
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

        let resolution = GameResolution {
            width: DEFAULT_WIDTH as f64,
            height: DEFAULT_HEIGHT as f64,
            dpi: 2.,
        };

        Self {
            render_state: RenderState::new(projection, view, resolution),
            camera,
        }
    }
}
