use super::{
    opengl,
    shaders::{self, ShaderType},
    Font, LoadedMesh, SunLight, Text,
};
use crate::colliders::{BoundingBox, Collider};
use crate::global::*;
use nalgebra_glm as glm;
use std::ptr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points,
}

// obb
pub fn draw_bbox(object: &LoadedMesh, bbox_mesh: &LoadedMesh) {
    let prog_id = SHADERS.activate(ShaderType::SimpleShader);
    opengl::use_vao(bbox_mesh.gpu_bound.vao);

    let (pos, rotation, scale) = object.transform.to_glm();

    let identity = glm::identity();
    let scale_matrix = glm::scale(&identity, &scale);
    let rotation_matrix = {
        let mut matrix = glm::rotate_x(&identity, rotation.x);
        matrix = glm::rotate_y(&matrix, rotation.y);
        matrix = glm::rotate_z(&matrix, rotation.z);

        matrix
    };

    let position_matrix = glm::translate(&identity, &pos);

    if object.collider.is_none() {
        return;
    }

    let bb = object.collider.unwrap().get_bb();
    let size = bb.size.to_glm();
    let center = bb.center.to_glm();

    let bbox_model = glm::translate(&glm::identity(), &center)
        * glm::scale(&identity, &(size * 1.1));

    let model = scale_matrix * rotation_matrix * position_matrix * bbox_model;
    shaders::set_matrix4(prog_id, "model", model.as_slice());

    // Set shader flags.
    bbox_mesh.flags.set_flags_to_shader(prog_id);

    let ebo = bbox_mesh.gpu_bound.ebo.unwrap();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::DrawElements(
            gl::TRIANGLES,
            bbox_mesh.gpu_bound.primitives_len as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );

        // Clear
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
}

pub fn draw_mesh(mesh: &LoadedMesh) {
    if mesh.is_hidden {
        return;
    }

    let gpu_bound = &mesh.gpu_bound;

    let prog_id = SHADERS.activate(ShaderType::SimpleShader);
    opengl::use_vao(gpu_bound.vao);

    shaders::set_bool(prog_id, "is_hover", mesh.is_hover);
    if mesh.is_hover {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    let (pos, rotation, scale) = mesh.transform.to_glm();

    let identity = glm::identity();
    let scale_matrix = glm::scale(&identity, &scale);
    let rotation_matrix = {
        let mut matrix = glm::rotate_x(&identity, rotation.x);
        matrix = glm::rotate_y(&matrix, rotation.y);
        matrix = glm::rotate_z(&matrix, rotation.z);

        matrix
    };
    let position_matrix = glm::translate(&identity, &pos);

    let model = scale_matrix * rotation_matrix * position_matrix;
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
        gl::Disable(gl::BLEND);
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
