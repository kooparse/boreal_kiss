use crate::global::*;
use super::{
    opengl,
    shaders::{self, ShaderType},
    Font, LoadedMesh, SunLight, Text,
};
use nalgebra_glm as glm;
use std::ptr;

#[derive(Copy, Clone, PartialEq)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points,
}

pub fn draw_mesh(mesh: &LoadedMesh) {
    // Global scope.
    let view = VIEW_MATRIX.lock().unwrap();
    let projection = PERSPECTIVE_MATRIX.lock().unwrap();

    if mesh.is_hidden {
        return;
    }

    let gpu_bound = &mesh.gpu_bound;

    let prog_id = SHADERS.activate(ShaderType::SimpleShader);
    opengl::use_vao(gpu_bound.vao);

    shaders::set_matrix4(prog_id, "view", view.as_slice());

    // TODO: Don't set projection matrix in the render loop.
    shaders::set_matrix4(prog_id, "projection", projection.as_slice());

    // TODO: This should be (maybe) stored in the object.
    let mut model = glm::Mat4::identity();
    model = glm::translate(&model, &mesh.world_pos.to_glm());

    shaders::set_matrix4(prog_id, "model", model.as_slice());

    // Set shader flags.
    mesh.flags.set_flags_to_shader(prog_id);

    gpu_bound
        .tex_ids
        .iter()
        .enumerate()
        .for_each(|(index, tex_id)| {
            shaders::set_sampler(prog_id, index);
            opengl::bind_texture(*tex_id, index);
        });

    unsafe {
        gl::Disable(gl::BLEND);

        match mesh.mode {
            DrawMode::Triangles => {
                if let Some(ebo) = gpu_bound.ebo {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        gpu_bound.primitives_len as i32,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    );
                } else {
                    gl::DrawArrays(
                        gl::TRIANGLES,
                        0,
                        gpu_bound.primitives_len as i32,
                    );
                }
            }

            DrawMode::Lines => {
                gl::DrawArrays(gl::LINES, 0, gpu_bound.primitives_len as i32);
            }

            _ => unimplemented!(),
        }
    }
}

// Draw directional sun light over the scene.
pub fn draw_sun_light(_light_source: &SunLight) {
    // draw stuff.
}

// Render some text to the screen.
// Used only for the editor/UI for now.
pub fn draw_text(font: &mut Font, text: &Text) {
    // Activate the text shader.
    let prog_id = SHADERS.activate(ShaderType::TextShader);
    opengl::use_shader_program(prog_id);

    shaders::set_matrix4(
        prog_id,
        "projection",
        ORTHO_MATRIX.lock().unwrap().as_slice(),
    );

    font.render(text);
}
