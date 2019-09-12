use super::object::{RendererObject, Vertex};
use super::shaders::{ShaderProgramId, ShaderType};
use super::texture::{Texture, UV};
use super::Color;
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
pub fn load_bytes_to_gpu(vbo: VBO, ebo: Option<EBO>, object: &RendererObject) {
    let mut total_size = object.vertices.data.len() * mem::size_of::<Vertex>();

    if let Some(texture) = &object.texture {
        total_size += texture.uv.len() * mem::size_of::<UV>();
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            total_size as isize,
            ptr::null() as *const _,
            gl::STATIC_DRAW,
        );

        // Vertex data.
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            (object.vertices.data.len() * mem::size_of::<Vertex>()) as isize,
            object.vertices.data.as_ptr() as *const _,
        );

        // Texture data.
        if let Some(texture) = &object.texture {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                (object.vertices.data.len() * mem::size_of::<Vertex>())
                    as isize,
                (texture.uv.len() * mem::size_of::<UV>()) as isize,
                texture.uv.as_ptr() as *const _,
            );
        }

        // Create EBO if indices is not empty.
        if let Some(ebo) = ebo {
            let indices = &object.vertices.indices;

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<f32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }
}

pub fn generate_texture() -> TexId {
    unsafe {
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        texture
    }
}

pub unsafe fn load_tex_to_gpu(tex: &Texture) {
    let dim = &tex.dim;
    let data = &tex.raw;

    gl::BindTexture(gl::TEXTURE_2D, tex.id);

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
        dim.width as i32,
        dim.height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const c_void,
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);
}

/// Use a given vao then load data to the gpu.
pub fn load_object_to_gpu(
    (vao, vbo, ebo): (VAO, VBO, Option<EBO>),
    object: &RendererObject,
) {
    unsafe {
        use_vao(vao);
        load_bytes_to_gpu(vbo, ebo, &object);

        match object.shader_type {
            ShaderType::SimpleShader => {
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    mem::size_of::<Vertex>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);
            }

            ShaderType::SimpleTextureShader => {
                load_tex_to_gpu(
                    &object
                        .texture
                        .as_ref()
                        .expect("Crash: texture object is not set"),
                );

                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    mem::size_of::<Vertex>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    mem::size_of::<UV>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(1);
            }
        }
    }
}

pub fn use_shader_program(id: ShaderProgramId) {
    unsafe {
        gl::UseProgram(id);
    }
}
