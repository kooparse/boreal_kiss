mod debug_camera;

use crate::global::*;
use crate::input::{Input, Key, MouseButton};
use crate::platform::WinitPlatform;
use crate::entities::Entities;
use crate::renderer::{Renderer, Text, Vector};
use crate::time::{Time, Timer};
use debug_camera::Camera;

#[derive(Default)]
pub struct Editor {
    pub camera: Camera,
    timer: Timer,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(0.5),
            ..Default::default()
        }
    }

    pub fn run(
        &mut self,
        entities: &mut Entities,
        platform: &WinitPlatform,
        input: &mut Input,
        renderer: &mut Renderer,
        time: &Time,
    ) {

        if input.modifiers.shift && input.is_pressed_once(Key::P) {
            renderer.toggle_wireframe();
        };

        // Timer to smooth debug text.
        if self.timer.is_passed(time.dt, 0.15) {
            // TODO: Careful here... we flush all the GUI text per frame.
            entities.text_widgets.flush();

            let content =
                format!("Meshes rendered: {}", renderer.debug_info.draw_call);
            let (x, y) = unsafe {
                (0.01 * SCREEN_WIDTH, 0.86 * SCREEN_HEIGHT)
            };
            entities.text_widgets.insert(Text {
                position: Vector(x, y, 0.),
                font_size: 31.,
                content, 
                ..Text::default()
            });


            let content = format!("Tick: {} ms", (time.dt * 1000.).round());
            let (x, y) = unsafe {
                (0.01 * SCREEN_WIDTH, 0.9 * SCREEN_HEIGHT)
            };
            entities.text_widgets.insert(Text {
                position: Vector(x, y, 0.),
                font_size: 31.,
                content, 
                ..Text::default()
            });
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
