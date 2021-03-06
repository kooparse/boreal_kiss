use super::mesh::{Vertex, UV};
use super::shaders::{ShaderProgramId, ShaderType};
use super::texture::Texture;
use super::types::Rgba;
use super::Vector;
use gl;
use std::{ffi::c_void, mem, ptr, str};

pub type VAO = u32;
pub type VBO = u32;
pub type EBO = u32;
pub type TexId = u32;

/// All the data linked to our backend renderer.
#[derive(Debug, Clone)]
pub struct GpuBound {
    pub vao: VAO,
    pub vbo: VBO,
    pub ebo: Option<EBO>,
    pub tex_ids: Vec<TexId>,
    pub primitives_len: usize,
    pub shader: ShaderType,
}

impl Drop for GpuBound {
    fn drop(&mut self) {
        unsafe {
            // Delete VAO.
            gl::DeleteVertexArrays(1, [self.vao].as_ptr());

            // Delete texture.
            gl::DeleteTextures(
                self.tex_ids.len() as i32,
                self.tex_ids.as_ptr(),
            );

            // Delete VBO and EBO.
            if let Some(ebo) = self.ebo {
                gl::DeleteBuffers(2, [self.vbo, ebo].as_ptr());
            } else {
                gl::DeleteBuffers(1, [self.vbo].as_ptr());
            }
        }
    }
}

/// Used to check if opengl is loaded (crash otherwise).
/// The method "slice_from_raw_parts" from the nightly would
/// be useful (https://doc.rust-lang.org/std/ptr/fn.slice_from_raw_parts.html).
/// TODO: This is very dangerous because sometimes it won't segfault
/// if pointer's values are u8 bu any chance and no gl is loaded.
pub fn get_opengl_loaded() {
    unsafe {
        let mem_ptr = gl::GetString(gl::VERSION);
        let version: [u8; 3] = [
            ptr::read(mem_ptr),
            ptr::read(mem_ptr.offset(1)),
            ptr::read(mem_ptr.offset(2)),
        ];

        let _version = str::from_utf8(&version)
            .expect("Error while retrieving the opengl version");

        println!("OpenGl version: {}", _version);
    }
}

#[allow(dead_code)]
pub fn set_viewport(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

/// Set multisampling.
pub fn set_multisampling(enabled: bool) {
    unsafe {
        if enabled {
            gl::Enable(gl::MULTISAMPLE);
        } else {
            gl::Disable(gl::MULTISAMPLE);
        }
    }
}
/// Set depth testing.
pub fn set_depth_testing(enabled: bool) {
    unsafe {
        if enabled {
            gl::Enable(gl::DEPTH_TEST);
        } else {
            gl::Disable(gl::DEPTH_TEST);
        }
    }
}

pub fn clear(color: &Rgba) {
    unsafe {
        gl::ClearColor(color.r, color.g, color.b, color.a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn gen_vao() -> VAO {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        vao
    }
}

pub fn gen_buffer() -> u32 {
    unsafe {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);
        id
    }
}

pub fn generate_ubo(size: usize, block_index: u32) -> u32 {
    unsafe {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);
        gl::BindBuffer(gl::UNIFORM_BUFFER, id);
        gl::BufferData(
            gl::UNIFORM_BUFFER,
            size as isize,
            ptr::null(),
            gl::STATIC_DRAW,
        );

        gl::BindBufferBase(gl::UNIFORM_BUFFER, block_index, id);
        // Unbind.
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

        id
    }
}

pub fn set_ubo<T>(ubo: u32, offset: isize, value: T) {
    unsafe {
        gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);

        gl::BufferSubData(
            gl::UNIFORM_BUFFER,
            offset,
            mem::size_of::<T>() as isize,
            &value as *const T as *const _,
        );

        // Unbind.
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
    }
}

pub fn use_vao(vao: VAO) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}

/// This create an vertex buffer object and load data.
pub fn load_bytes_to_gpu(vao: VAO, vertex: &Vertex) -> (VBO, Option<EBO>) {
    let with_ebo = !vertex.indices.is_empty();

    let mut total_size = (vertex.primitives.len() * mem::size_of::<Vector>())
        + (vertex.colors.len() * mem::size_of::<Rgba>());

    vertex.uv_coords.iter().for_each(|set| {
        total_size += set.coords.len() * mem::size_of::<UV>();
    });

    unsafe {
        use_vao(vao);
        let vbo = gen_buffer();
        let ebo = if with_ebo { Some(gen_buffer()) } else { None };

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            total_size as isize,
            ptr::null() as *const _,
            gl::STATIC_DRAW,
        );

        let mut data_cursor: isize = 0;

        // Vector data.
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            data_cursor,
            (vertex.primitives.len() * mem::size_of::<Vector>()) as isize,
            vertex.primitives.as_ptr() as *const _,
        );
        data_cursor +=
            (vertex.primitives.len() * mem::size_of::<Vector>()) as isize;

        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            data_cursor,
            (vertex.colors.len() * mem::size_of::<Rgba>()) as isize,
            vertex.colors.as_ptr() as *const _,
        );

        data_cursor += (vertex.colors.len() * mem::size_of::<Rgba>()) as isize;

        // Texture data.
        vertex.uv_coords.iter().for_each(|uv| {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                data_cursor,
                (uv.coords.len() * mem::size_of::<UV>()) as isize,
                uv.coords.as_ptr() as *const _,
            );
            data_cursor += (uv.coords.len() * mem::size_of::<UV>()) as isize;
        });

        // Create EBO if indices is not empty.
        if let Some(ebo) = ebo {
            let indices = &vertex.indices;

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<f32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }

        (vbo, ebo)
    }
}

pub fn generate_texture() -> TexId {
    unsafe {
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        texture
    }
}

pub fn bind_texture(tex_id: TexId, texture_number: usize) {
    unsafe {
        // TEXTURE0 + 1 = TEXTURE1.
        gl::ActiveTexture(gl::TEXTURE0 + texture_number as u32);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
    }
}

pub unsafe fn load_tex_to_gpu(vao: VAO, tex: &Texture, is_font: bool) -> TexId {
    let dim = &tex.dim;
    let data = &tex.raw;

    use_vao(vao);
    let tex_id = generate_texture();

    let color_format = gl::RGBA;

    let clamp = if is_font {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl::CLAMP_TO_EDGE
    } else {
        gl::REPEAT
    } as i32;

    gl::BindTexture(gl::TEXTURE_2D, tex_id);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, clamp);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, clamp);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as i32,
    );

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        color_format as i32,
        dim.0 as i32,
        dim.1 as i32,
        0,
        color_format,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const c_void,
    );

    if !is_font {
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    tex_id
}

pub fn load_font_to_gpu(
    vertices: &Vec<f32>,
    texture_atlas: &Texture,
) -> (VAO, VBO, TexId) {
    let vao = gen_vao();
    use_vao(vao);

    unsafe {
        let tex_id = load_tex_to_gpu(vao, texture_atlas, true);
        let vbo = gen_buffer();

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * mem::size_of::<f32>()) as _,
            ptr::null(),
        );

        gl::EnableVertexAttribArray(0);

        (vao, vbo, tex_id)
    }
}

/// Use a given vao then load data to the gpu.
pub fn load_object_to_gpu(
    vertex: &Vertex,
    textures: &Vec<Texture>,
) -> (VAO, VBO, Option<EBO>, Vec<TexId>) {
    unsafe {
        let vao = gen_vao();
        let (vbo, ebo) = load_bytes_to_gpu(vao, vertex);

        let tex_ids = textures
            .iter()
            .map(|tex| load_tex_to_gpu(vao, &tex, false))
            .collect();

        use_vao(vao);

        let mut location = 0;
        let mut data_cursor = 0;

        // Vectors
        gl::VertexAttribPointer(
            location,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vector>() as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        location += 1;
        data_cursor += vertex.primitives.len() * mem::size_of::<Vector>();

        // Colors
        gl::VertexAttribPointer(
            location,
            4,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Rgba>() as i32,
            data_cursor as *const _,
        );
        gl::EnableVertexAttribArray(1);

        location += 1;
        data_cursor += vertex.colors.len() * mem::size_of::<Rgba>();

        // UV Coords.
        for set in vertex.uv_coords.iter() {
            gl::VertexAttribPointer(
                location,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<UV>() as i32,
                data_cursor as *const _,
            );
            gl::EnableVertexAttribArray(location);

            location += 1;
            data_cursor += set.coords.len() * mem::size_of::<UV>();
        }

        (vao, vbo, ebo, tex_ids)
    }
}

pub fn use_shader_program(id: ShaderProgramId) {
    unsafe {
        gl::UseProgram(id);
    }
}
