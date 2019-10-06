mod debug_scenes;

use engine::{
    camera::Camera,
    input::{Cursor, Input, Key, MouseButton},
    platform::PlatformWrapper,
    time::Time,
};
use nalgebra_glm as glm;
use renderer::{
    storage::GenerationId, 
    Pos2D, 
    RenderState, 
    Renderer, 
    color::Rgb
};

#[derive(Default)]
pub struct Editor {
    pub camera: Camera,
    debug_text_id: GenerationId,
    is_debug_box: bool,
}

impl Editor {
    pub fn init(&mut self, renderer: &mut Renderer) {
        let scene = debug_scenes::scene_mesh();
        renderer.flush_meshes();
        renderer.add_meshes(scene);

        self.debug_text_id =
            renderer.add_text("", Pos2D::default(), Rgb::default());
    }

    pub fn update_events(
        &mut self,
        input: &mut Input,
        renderer: &mut Renderer,
        r_state: &RenderState,
        _time: &Time,
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

        input.clicked(MouseButton::Left, |cursor| {
            let (origin, direction) = self.cast_ray(cursor, r_state);
            renderer.add_ray(origin, direction, 100f32);
        });

        let (_, height) = (
            r_state.resolution.width as f32,
            r_state.resolution.height as f32,
        );

        let mesh_nb =
            format!("Meshes rendered: {}", renderer.debug_info.mesh_call_nb);

        renderer.update_text(
            self.debug_text_id,
            mesh_nb,
            Pos2D(5., height - 40.),
        );

        input.pressed_once(Key::M, || {
            if self.is_debug_box {
                renderer.remove_text(&self.debug_text_id);
                self.is_debug_box = false;
            } else {
                self.debug_text_id = renderer.add_text(
                    "Il était une fois",
                    Pos2D(20., 20.),
                    Rgb::new(1., 1., 1.),
                );

                self.is_debug_box = true;
            }
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
