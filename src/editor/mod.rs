mod debug_camera;

use crate::colliders::*;
use crate::entities::{Entities, Entity, Handle};
use crate::global::*;
use crate::input::{Input, Key, MouseButton};
use crate::platform::WinitPlatform;
use crate::renderer::{Mesh, Renderer, Text, Vector};
use crate::time::{Time, Timer};
use debug_camera::Camera;
use nalgebra_glm as glm;

#[derive(PartialEq, Debug)]
enum ObjectTransformMode {
    Position,
    Rotation,
    Scale,
}

pub struct Editor {
    pub camera: Camera,
    timer: Timer,
    object_mode: ObjectTransformMode,
    selected_handle: Option<Handle<Mesh>>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(0.5),
            camera: Camera::default(),
            object_mode: ObjectTransformMode::Position,
            selected_handle: None,
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

        if input.is_pressed_once(Key::J) {
            dbg!(self.camera.front);
            dbg!(self.camera.position);
        }

        // Permute select mode
        // if input.modifiers.shift {
        //     if input.is_pressed_once(Key::L) {
        //         self.object_mode = match self.object_mode {
        //             ObjectTransformMode::Position => {
        //                 ObjectTransformMode::Rotation
        //             }
        //             ObjectTransformMode::Rotation => ObjectTransformMode::Scale,
        //             ObjectTransformMode::Scale => ObjectTransformMode::Position,
        //         };
        //     }
        // }

        // Timer to smooth debug text.
        if self.timer.is_passed(time.dt, 0.1) {
            // TODO: Careful here... we flush all the GUI text per frame.
            entities.text_widgets.flush();
            let (x, y) = unsafe { (0.01 * SCREEN_WIDTH, SCREEN_HEIGHT) };

            let content = format!("Frame: {} ms", (time.dt * 1000.).round());
            entities.insert(Text {
                position: Vector(x, y * 0.9, 0.),
                font_size: 31.,
                content,
                ..Text::default()
            });

            let content =
                format!("Meshes rendered: {}", renderer.debug_info.draw_call);
            entities.insert(Text {
                position: Vector(x, y * 0.86, 0.),
                font_size: 31.,
                content,
                ..Text::default()
            });

            entities.insert(Text {
                position: Vector(x, y * 0.82, 0.),
                font_size: 31.,
                content: format!("Mode: {:?}", self.object_mode),
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

        // Mouse picking...
        let view_matrix = *VIEW_MATRIX.lock().unwrap();
        let proj_matrix = *PERSPECTIVE_MATRIX.lock().unwrap();
        // Viewport space.
        let screen_point = input.cursor.position;
        let cam_pos = self.camera.position;

        unsafe {
            let dpi = SCREEN_DPI as f32;

            let ray_ndc = glm::vec3(
                (screen_point.0 as f32 * dpi) / SCREEN_WIDTH - 1.,
                1. - (screen_point.1 as f32 * dpi) / SCREEN_HEIGHT,
                1.,
            );
            let ray_clip = glm::vec4(ray_ndc.x, ray_ndc.y, -1., 1.);
            let ray_eye = glm::inverse(&proj_matrix) * ray_clip;
            let ray_eye = glm::vec4(ray_eye.x, ray_eye.y, -1., 0.);
            let ray_world = (glm::inverse(&view_matrix) * ray_eye).xyz();

            // Deplace a bit the origin, otherwise we have
            // to move before seeing the ray.
            let origin = cam_pos;
            let direction = glm::normalize(&ray_world);

            // All meshes
            let entity_handles: Vec<Handle<Mesh>> =
                entities.meshes.iter().map(|(_, h)| *h).collect();

            let mut hit_array: Vec<(Handle<Mesh>, f32)> = vec![];

            let mouse_is_hold = input.is_clicked(MouseButton::Left);
            let mouse_is_down = input.is_clicked_once(MouseButton::Left);

            for handle in entity_handles {
                let entity = entities.get_mut(&handle);
                let is_current =
                    self.selected_handle.map_or(false, |sh| sh == handle);

                ray_intersect_aabb((origin, direction), entity);
                let (is_hit, t) =
                    entity.collider.map_or((false, 0.), |collider| {
                        match &collider {
                            Collider::Plane => {
                                plane_hit((origin, direction), entity)
                            }
                            Collider::Sphere => {
                                sphere_hit((origin, direction), entity)
                            }
                            Collider::Cube => {
                                intersect_ray_box((origin, direction), entity)
                            }
                        }
                    });

                // Hover objects.
                entity.is_hover = !is_current && is_hit;

                if !is_hit && is_current && mouse_is_down {
                    self.selected_handle = None;
                }

                if is_hit && mouse_is_hold {
                    hit_array.push((handle, t));
                } else {
                    entity.is_selected = false;
                }
            }

            // Sort by nearest.
            hit_array.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

            // If no current selected, we take the nearest.
            if self.selected_handle.is_none() && !hit_array.is_empty() {
                self.selected_handle = Some(hit_array[0].0);
            }

            if let Some(handle) = self.selected_handle {
                self.selected_handle = Some(handle);

                let entity = entities.get_mut(&handle);
                entity.is_selected = true;

                if mouse_is_hold && input.cursor.has_moved {
                    entity.is_dragged = true;
                } else {
                    entity.is_dragged = false;
                    return;
                }

                let (delta_x, delta_y) = input.cursor.delta;
                let delta_x = (delta_x * time.dt) as f32;
                let delta_y = (delta_y * time.dt) as f32;

                match self.object_mode {
                    ObjectTransformMode::Position => {
                        let pos_ptr = &mut entity.transform.position;

                        if input.modifiers.shift {
                            pos_ptr.1 -= delta_y;
                            return;
                        }

                        pos_ptr.0 += delta_x;
                        pos_ptr.2 += delta_y;
                    }
                    ObjectTransformMode::Rotation => {
                        let rotation_ptr = &mut entity.transform.rotation;
                        if input.modifiers.shift {
                            rotation_ptr.0 += delta_y;
                            return;
                        }
                        rotation_ptr.2 += delta_x;
                    }
                    ObjectTransformMode::Scale => {
                        let scale_ptr = &mut entity.transform.scale;
                        scale_ptr.0 += delta_x;
                        scale_ptr.1 += delta_x;
                        scale_ptr.2 += delta_x;
                    }
                }
            }
        }
    }
}
