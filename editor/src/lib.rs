mod debug_scenes;

use engine::{
    camera::Camera,
    input::{Cursor, Input, Key, MouseButton},
    platform::PlatformWrapper,
    time::{Time, Timer},
};
use nalgebra_glm as glm;
use renderer::{GenerationId, RenderState, Renderer, Rgb, Text, Vector};

#[derive(Default)]
pub struct Editor {
    pub camera: Camera,
    text_dt_id: GenerationId,
    text_mesh_id: GenerationId,
    timer: Timer,
}

impl Editor {
    pub fn init(&mut self, renderer: &mut Renderer) {
        let scene = debug_scenes::scene_mesh();
        renderer.flush_meshes();
        renderer.add_meshes(scene);

        self.timer = Timer::new(0.5);

        self.text_mesh_id = renderer.add_text(Text {
            position: Vector(10., 750., 0.),
            font_size: 31.,
            ..Text::default()
        });

        self.text_dt_id = renderer.add_text(Text {
            position: Vector(10., 725., 0.),
            font_size: 31.,
            ..Text::default()
        });
    }

    pub fn update_events(
        &mut self,
        input: &mut Input,
        renderer: &mut Renderer,
        r_state: &RenderState,
        time: &Time,
    ) {
        input.pressed_once(Key::Key1, || {
            renderer.flush_meshes();
            let scene = debug_scenes::scene_mesh();
            renderer.add_meshes(scene);
        });

        input.pressed_once(Key::Key2, || {
            renderer.flush_meshes();
            let scene = debug_scenes::scene_light();
            renderer.add_meshes(scene);
        });

        input.pressed_once(Key::Key0, || {
            renderer.toggle_wireframe();
        });

        input.clicked(MouseButton::Left, |cursor| {
            let (origin, direction) = self.cast_ray(cursor, r_state);
            renderer.add_ray(origin, direction, 100f32);
        });

        if self.timer.is_passed(time.dt, 0.15) {
            renderer.update_text(self.text_mesh_id).content =
                format!("Meshes rendered: {}", renderer.debug_info.draw_call);

            let text_dt_id = (time.dt * 1000.).round();
            let framerate_text = renderer.update_text(self.text_dt_id);

            if text_dt_id > 16. {
                framerate_text.color = Rgb::new(1., 0., 0.);
            }

            framerate_text.content = format!("Tick: {} ms", text_dt_id);
        }
    }

    pub fn move_camera(
        &mut self,
        input: &mut Input,
        window: &dyn PlatformWrapper,
        time: &Time,
    ) -> glm::Mat4 {
        if input.modifiers.shift {
            window.hide_cursor(true);
            self.camera.update(input, time);
        } else {
            window.hide_cursor(false);
        }

        self.camera.get_look_at()
    }

    pub fn cast_ray(
        &self,
        cursor: &Cursor,
        render_state: &RenderState,
    ) -> (Vector, Vector) {
        let cam_pos = self.camera.position;

        let screen_point = cursor.position;
        let res = &render_state.resolution;
        let viewport = glm::vec4(0., 0., res.width as f32, res.height as f32);

        let far_ndc_point = glm::vec3(
            screen_point.0 as f32 / res.width as f32,
            screen_point.1 as f32 / res.height as f32,
            0.0,
        );

        let far_view_point = glm::unproject_no(
            &far_ndc_point,
            &render_state.view,
            &render_state.projection,
            viewport,
        );

        (
            Vector::from_glm(cam_pos),
            Vector::from_glm(glm::normalize(&(-far_view_point))),
        )
    }
}
