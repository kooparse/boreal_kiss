use crate::input::{Input, Key};
use crate::player::Player;
use crate::time::Time;

use nalgebra_glm as glm;

#[derive(Debug, Clone)]
pub struct Camera {
    pub rotation: CamRotation,
    pub position: glm::TVec3<f32>,
    pub target_pos: glm::TVec3<f32>,
    pub up: glm::TVec3<f32>,
    pub threshold: glm::TVec3<f32>,
}

impl Camera {
    pub fn new(player: &Player) -> Self {
        let player_pos = glm::vec3(
            player.world_pos.x,
            player.world_pos.y,
            player.world_pos.z,
        );

        Self {
            target_pos: player_pos,
            ..Self::default()
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            rotation: CamRotation::Behind,
            position: glm::vec3(0., 0., 0.),
            target_pos: glm::vec3(0., 0., 0.),
            up: glm::vec3(0., 1., 0.),
            threshold: glm::vec3(0., 6., 0.),
        }
    }
}

impl Camera {
    pub fn follow_player(
        &mut self,
        player: &Player,
        input: &mut Input,
        time: &Time,
    ) -> glm::TMat4<f32> {
        let player_pos = glm::vec3(
            player.world_pos.x,
            player.world_pos.y,
            player.world_pos.z,
        );

        if input.is_pressed_once(Key::H) {
            self.rotation = self.rotation.rotate_add();
        };

        if input.is_pressed_once(Key::L) {
            self.rotation = self.rotation.rotate_sub();
        };

        let end_pos = player_pos + self.threshold + self.rotation.rotate(10.);
        let speed = 5. * time.dt as f32;
        let a = glm::vec3(speed, 1., speed);

        self.target_pos = glm::lerp_vec(&self.target_pos, &player_pos, &a);
        self.position = glm::lerp_vec(&self.position, &end_pos, &a);

        glm::look_at(&self.position, &self.target_pos, &self.up)
    }
}

#[derive(Debug, Clone)]
pub enum CamRotation {
    Behind,
    Forward,
    FromRight,
    FromLeft,
}

impl CamRotation {
    fn rotate_add(&self) -> Self {
        match self {
            Self::Behind => Self::FromLeft,
            Self::FromLeft => Self::Forward,
            Self::Forward => Self::FromRight,
            Self::FromRight => Self::Behind,
        }
    }

    fn rotate_sub(&self) -> Self {
        match self {
            Self::Behind => Self::FromRight,
            Self::FromRight => Self::Forward,
            Self::Forward => Self::FromLeft,
            Self::FromLeft => Self::Behind,
        }
    }

    fn rotate(&self, distance: f32) -> glm::TVec3<f32> {
        let mut padding = glm::vec3(0., 0., 0.);

        match self {
            Self::Behind => padding.z = -1.,
            Self::FromRight => padding.x = -1.,
            Self::Forward => padding.z = 1.,
            Self::FromLeft => padding.x = 1.,
        };

        padding * distance
    }
}
