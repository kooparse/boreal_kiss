mod debug_scenes;

use engine::{
    camera::Camera,
    input::{Cursor, Input, Key, MouseButton},
    platform::PlatformWrapper,
    time::Time,
};
use nalgebra_glm as glm;
use renderer::{RenderState, Renderer};

#[derive(Default)]
pub struct Editor {
    pub camera: Camera,
}

impl Editor {
    pub fn init(&self, renderer: &mut Renderer) {
        let scene = debug_scenes::scene_mesh();
        renderer.flush();
        renderer.add_meshes(scene);
    }

    pub fn update_events(
        &self,
        input: &mut Input,
        renderer: &mut Renderer,
        r_state: &RenderState,
    ) {
        input.pressed_once(Key::Key1, || {
            renderer.flush();
            let scene = debug_scenes::scene_mesh();
            renderer.add_meshes(scene);
        });

        input.pressed_once(Key::Key2, || {
            renderer.flush();
            let scene = debug_scenes::scene_light();
            renderer.add_meshes(scene);
        });

        input.clicked(MouseButton::Left, |cursor| {
            let (origin, direction) = self.cast_ray(cursor, r_state);
            renderer.add_ray(origin, direction, 100f32);
        });
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
    ) -> (glm::TVec3<f32>, glm::TVec3<f32>) {
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

        (cam_pos, glm::normalize(&(-far_view_point)))
    }
}
