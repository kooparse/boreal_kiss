mod debug_camera;

use crate::global::*;
use crate::input::{Input, Key, MouseButton};
use crate::platform::WinitPlatform;
use crate::renderer::{GenerationId, Renderer, Text, Vector};
use crate::time::{Time, Timer};
use debug_camera::Camera;

#[derive(Default)]
pub struct Editor {
    pub camera: Camera,
    text_dt_id: GenerationId,
    text_mesh_id: GenerationId,
    timer: Timer,
}

impl Editor {
    pub fn new(renderer: &mut Renderer) -> Self {
        Self {
            text_dt_id: renderer.add_text(Text {
                position: Vector(10., 725., 0.),
                font_size: 31.,
                ..Text::default()
            }),
            text_mesh_id: renderer.add_text(Text {
                position: Vector(10., 750., 0.),
                font_size: 31.,
                ..Text::default()
            }),

            timer: Timer::new(0.5),
            ..Default::default()
        }
    }

    pub fn run(
        &mut self,
        platform: &WinitPlatform,
        input: &mut Input,
        renderer: &mut Renderer,
        time: &Time,
    ) {
        // Toggle wireframe mode on this key when "0" is pressed.
        if input.is_pressed_once(Key::Key0) {
            renderer.toggle_wireframe();
        };

        // Timer to smooth debug text.
        if self.timer.is_passed(time.dt, 0.15) {
            renderer.update_text(self.text_mesh_id).content =
                format!("Meshes rendered: {}", renderer.debug_info.draw_call);

            let text_dt_id = (time.dt * 1000.).round();
            let framerate_text = renderer.update_text(self.text_dt_id);

            framerate_text.content = format!("Tick: {} ms", text_dt_id);
        }

        if input.is_clicked(MouseButton::Right) {
            platform.hide_cursor(true);
            self.camera.update(input, time);
            // Update the view matrix once the camera has moved.
            *VIEW_MATRIX.lock().unwrap() = self.camera.get_look_at();
        } else {
            platform.hide_cursor(false);
        }

        // if input.is_clicked_once(MouseButton::Left) {
        //     let (origin, direction) = self.cast_ray(&input.cursor, r_state);
        //     renderer.add_ray(origin, direction, 100f32);
        // }
    }

    // pub fn cast_ray(
    //     &self,
    //     cursor: &Cursor,
    //     render_state: &RenderState,
    // ) -> (Vector, Vector) {
    //     let cam_pos = self.camera.position;

    //     let screen_point = cursor.position;
    //     let res = &render_state.resolution;
    //     let viewport = glm::vec4(0., 0., res.width as f32, res.height as f32);

    //     let far_ndc_point = glm::vec3(
    //         screen_point.0 as f32 / res.width as f32,
    //         screen_point.1 as f32 / res.height as f32,
    //         0.0,
    //     );

    //     let far_view_point = glm::unproject_no(
    //         &far_ndc_point,
    //         &render_state.view,
    //         &render_state.projection,
    //         viewport,
    //     );

    //     (
    //         Vector::from_glm(cam_pos),
    //         Vector::from_glm(glm::normalize(&(-far_view_point))),
    //     )
    // }
}
