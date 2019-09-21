use crate::{
    input::{Input, Key},
    time::Time,
};
use nalgebra_glm as glm;
use std::default::Default;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: glm::TVec3<f32>,

    pub pos: (f64, f64),
    pub speed: f64,
    pub target: glm::TVec3<f32>,
    pub front: glm::TVec3<f32>,
    pub up: glm::TVec3<f32>,

    pub pitch: f64,
    pub yaw: f64,
    pub first_mouse: bool,
    pub last_pos: (f64, f64),
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            speed: 2.5,
            position: glm::vec3(0.0, 1.0, 3.),
            pos: (0., 0.),
            front: glm::vec3(0., 0., -1.),
            target: glm::vec3(0., 0., 0.),
            up: glm::vec3(0., 1., 0.),

            first_mouse: true,
            pitch: 0.,
            yaw: -90.,
            last_pos: (0., 0.),
        }
    }
}

impl Camera {
    pub fn get_look_at(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn update(
        &mut self,
        input: &mut Input,
        time: &Time,
    ) -> glm::TMat4<f32> {
        self.update_pos(input, time);
        self.update_spin(input, time);

        self.get_look_at()
    }

    pub fn update_pos(&mut self, input: &mut Input, time: &Time) {
        let (speed, front, up) = (self.speed, self.front, self.up);

        let speed = (speed * time.dt) as f32;

        input.pressed(Key::W, || {
            self.position += speed * front;
        });

        input.pressed(Key::S, || {
            self.position -= speed * front;
        });

        input.pressed(Key::D, || {
            self.position += glm::normalize(&front.cross(&up)) * speed;
        });

        input.pressed(Key::A, || {
            self.position -= glm::normalize(&front.cross(&up)) * speed;
        });

        input.pressed(Key::Q, || {
            self.position -= speed * up;
        });

        input.pressed(Key::E, || {
            self.position += speed * up;
        });
    }

    pub fn update_spin(&mut self, input: &mut Input, time: &Time) {
        if !input.cursor.has_moved {
            return;
        }

        let (delta_x, delta_y) = input.cursor.delta;
        self.pos.0 += delta_x;
        self.pos.1 += delta_y;

        let (pos_x, pos_y) = self.pos;

        if self.first_mouse {
            self.last_pos = (pos_x, pos_y);
            self.first_mouse = false;
        }

        let mut x_offset = pos_x - self.last_pos.0;
        let mut y_offset = self.last_pos.1 - pos_y;
        self.last_pos = (pos_x, pos_y);

        x_offset *= self.speed * time.dt;
        y_offset *= self.speed * time.dt;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch > 89. {
            self.pitch = 89.;
        }
        if self.pitch < -89. {
            self.pitch = -89.;
        }

        let mut front = glm::vec3(0., 0., 0.);
        let yaw = self.yaw as f32;
        let pitch = self.pitch as f32;

        front.x = yaw.to_radians().cos() * pitch.to_radians().cos();
        front.y = pitch.to_radians().sin();
        front.z = yaw.to_radians().sin() * pitch.to_radians().cos();

        self.front = glm::normalize(&front);
    }
}
