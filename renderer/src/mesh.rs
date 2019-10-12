use nalgebra_glm as glm;
use std::cmp::min;
use std::ptr;
use crate::{
    Vector,
    opengl,
    shaders,
    DrawMode,
    GpuBound,
    RenderState,
    DrawableObject,
    texture::Texture,
    vertex::Vertex,
    shaders::{ShaderFlags, ShaderType, ShaderProgramId}
};

pub struct Mesh<'n> {
    pub name: &'n str,
    pub vertex: Vertex,
    pub textures: Vec<Texture>,
    pub shader_type: ShaderType,
    pub world_pos: Vector,
    pub mode: DrawMode,
}

impl<'n> From<&Mesh<'n>> for LoadedMesh {
    fn from(object: &Mesh<'n>) -> LoadedMesh {
        // From system memmory to gpu memory.
        let (vao, vbo, ebo, tex_ids) = opengl::load_object_to_gpu(&object);

        let primitives_len = ebo.map_or(object.vertex.primitives.len(), |_| {
            object.vertex.indices.len()
        });

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo,
            primitives_len,
            shader: object.shader_type,
            tex_ids,
        };

        let (has_uv, has_multi_uv, has_vert_colors, _tex_number) = {
            let colors = &object.vertex.colors;
            // We want a correlation between the number of set of coords
            // and the number of texture loaded.
            let tex_number =
                min(object.vertex.uv_coords.len(), object.textures.len());

            (
                tex_number > 0,
                tex_number > 1,
                !colors.is_empty(),
                tex_number,
            )
        };

        let flags = ShaderFlags {
            has_uv,
            has_multi_uv,
            has_vert_colors,
        };

        LoadedMesh {
            is_hidden: false,
            mode: object.mode,
            world_pos: object.world_pos,
            gpu_bound,
            flags,
        }
    }
}

pub struct LoadedMesh {
    pub is_hidden: bool,
    pub world_pos: Vector,
    pub mode: DrawMode,
    pub(crate) gpu_bound: GpuBound,
    pub flags: ShaderFlags,
}

impl DrawableObject for LoadedMesh {
    fn draw(&self, state: &RenderState, program_id: ShaderProgramId) {
            if self.is_hidden {
                return;
            }

            let gpu_bound = &self.gpu_bound;

            opengl::use_shader_program(program_id);
            opengl::use_vao(gpu_bound.vao);

            shaders::set_matrix4(
                program_id,
                "view",
                state.view.as_slice(),
            );

            // TODO: Don't set projection matrix in the render loop.
            shaders::set_matrix4(
                program_id,
                "projection",
                state.projection.as_slice(),
            );

            // TODO: This should be (maybe) stored in the object.
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &self.world_pos.to_glm());

            shaders::set_matrix4(program_id, "model", model.as_slice());

            // Set shader flags.
            self.flags.set_flags_to_shader(program_id);

            gpu_bound
                .tex_ids
                .iter()
                .enumerate()
                .for_each(|(index, tex_id)| {
                    shaders::set_sampler(program_id, index);
                    opengl::bind_texture(*tex_id, index);
                });

            unsafe {
                gl::Disable(gl::BLEND);
                // self.debug_info.draw_call += 1;

                match self.mode {
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
                        gl::DrawArrays(
                            gl::LINES,
                            0,
                            gpu_bound.primitives_len as i32,
                        );
                    }

                    _ => unimplemented!(),
                }
            }
    }

    fn cleanup(&self) {

    }
}

