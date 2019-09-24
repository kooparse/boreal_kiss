use crate::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use engine::{
    camera::Camera,
    input::{Cursor, Input},
    platform::PlatformWrapper,
    time::Time,
};
use nalgebra_glm as glm;
use renderer::{GameResolution, RenderState};

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

    // TODO: Remove this function from the "state".
    pub fn cast_ray(
        &self,
        cursor: &Cursor,
    ) -> (glm::TVec3<f32>, glm::TVec3<f32>) {
        let cam_pos = self.camera.position;

        let screen_point = cursor.position;
        let viewport =
            glm::vec4(0., 0., DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32);

        let far_ndc_point = glm::vec3(
            screen_point.0 as f32 / DEFAULT_WIDTH as f32,
            screen_point.1 as f32 / DEFAULT_HEIGHT as f32,
            0.0,
        );

        let far_view_point = glm::unproject_no(
            &far_ndc_point,
            &self.render_state.view,
            &self.render_state.projection,
            viewport,
        );

        (cam_pos, glm::normalize(&(-far_view_point)))
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
