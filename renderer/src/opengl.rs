use super::shaders::{ShaderProgramId, ShaderType};
use super::texture::Texture;
use super::vertex::Vector3;
use super::{Color, Mesh};
use gl;
use std::{ffi::c_void, mem, ptr, str};

pub type VAO = u32;
pub type VBO = u32;
pub type EBO = u32;
pub type TexId = u32;

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

        let version = str::from_utf8(&version)
            .expect("Error while retrieving the opengl version");

        println!("OpenGl version: {}", version);
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
        match enabled {
            true => gl::Enable(gl::MULTISAMPLE),
            false => gl::Disable(gl::MULTISAMPLE),
        }
    }
}
/// Set depth testing.
pub fn set_depth_testing(enabled: bool) {
    unsafe {
        match enabled {
            true => gl::Enable(gl::DEPTH_TEST),
            false => gl::Disable(gl::DEPTH_TEST),
        }
    }
}

pub fn clear(color: &Color) {
    unsafe {
        gl::ClearColor(color.0, color.1, color.2, color.3);
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

pub fn use_vao(vao: VAO) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}

/// This create an vertex buffer object and load data.
pub fn load_bytes_to_gpu(vao: VAO, object: &Mesh) -> (VBO, Option<EBO>) {
    let with_ebo = !object.vertex.indices.is_empty();

    let mut total_size =
        object.vertex.primitives.len() * mem::size_of::<Vector3>();

    if object.texture.is_some() {
        total_size += object.vertex.uv_coords.len() * mem::size_of::<Vector3>();
    }

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

        // Position data.
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            (object.vertex.primitives.len() * mem::size_of::<Vector3>())
                as isize,
            object.vertex.primitives.as_ptr() as *const _,
        );

        // Texture data.
        if object.texture.is_some() {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                (object.vertex.primitives.len() * mem::size_of::<Vector3>())
                    as isize,
                (object.vertex.uv_coords.len() * mem::size_of::<Vector3>())
                    as isize,
                object.vertex.uv_coords.as_ptr() as *const _,
            );
        }

        // Create EBO if indices is not empty.
        if let Some(ebo) = ebo {
            let indices = &object.vertex.indices;

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

pub unsafe fn load_tex_to_gpu(vao: VAO, tex: &Texture) -> TexId {
    let dim = &tex.dim;
    let data = &tex.raw;

    let tex_id = generate_texture();

    use_vao(vao);
    gl::BindTexture(gl::TEXTURE_2D, tex_id);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
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
        gl::RGBA as i32,
        dim.0 as i32,
        dim.1 as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const c_void,
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);

    tex_id
}

/// Use a given vao then load data to the gpu.
pub fn load_object_to_gpu(
    object: &Mesh,
) -> (VAO, VBO, Option<EBO>, Option<TexId>) {
    unsafe {
        let vao = gen_vao();

        let (vbo, ebo) = load_bytes_to_gpu(vao, &object);

        let tex_id = object
            .texture
            .as_ref()
            .map(|tex| load_tex_to_gpu(vao, &tex));

        use_vao(vao);

        match object.shader_type {
            ShaderType::SimpleShader => {
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    mem::size_of::<Vector3>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);
            }

            ShaderType::SimpleTextureShader => {
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    mem::size_of::<Vector3>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    mem::size_of::<Vector3>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(1);
            }
        }

        (vao, vbo, ebo, tex_id)
    }
}

pub fn use_shader_program(id: ShaderProgramId) {
    unsafe {
        gl::UseProgram(id);
    }
}
