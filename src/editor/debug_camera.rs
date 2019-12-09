use crate::input::{Input, Key};
use crate::time::Time;
use nalgebra_glm as glm;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: glm::TVec3<f32>,

    pub pos: (f64, f64),
    pub speed: f64,
    pub max_speed: f64,
    pub acceleration: f64,
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
            speed: 0.,
            max_speed: 2.5,
            acceleration: 0.3,

            position: glm::vec3(0.0, 4.0, 3.),
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

    pub fn look_at_player(&self, player_pos: glm::Vec3) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &player_pos, &self.up)
    }

    pub fn update(&mut self, input: &mut Input, time: &Time) {
        self.update_pos(input, time);
        self.update_spin(input, time);
    }

    pub fn update_pos(&mut self, input: &mut Input, time: &Time) {
        let (front, up) = (self.front, self.up);

        // Smooth start/end (even if end is not working currently).
        if input.is_nothing_pressed() {
            self.speed = 0.;
        } else {
            if self.speed < self.max_speed {
                self.speed += 0.3;
            } else {
                self.speed = self.max_speed;
            }
        }

        let speed = (self.speed * time.dt) as f32;

        if input.is_pressed(Key::W) {
            self.position += speed * front;
        };

        if input.is_pressed(Key::S) {
            self.position -= speed * front;
        };

        if input.is_pressed(Key::D) {
            self.position += glm::normalize(&front.cross(&up)) * speed;
        };

        if input.is_pressed(Key::A) {
            self.position -= glm::normalize(&front.cross(&up)) * speed;
        };

        if input.is_pressed(Key::Q) {
            self.position -= speed * up;
        };

        if input.is_pressed(Key::E) {
            self.position += speed * up;
        };
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

        x_offset *= self.max_speed * time.dt;
        y_offset *= self.max_speed * time.dt;

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
