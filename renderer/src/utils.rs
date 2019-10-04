use super::RenderState;
use nalgebra_glm as glm;

pub fn ortho_proj(state: &RenderState) -> glm::Mat4 {
        glm::ortho(
            0.,
            state.resolution.width as f32,
            0.,
            state.resolution.height as f32,
            -1.,
            1.,
        )
}

