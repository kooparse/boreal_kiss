use super::shaders::ShaderProgramId;
use super::texture::Texture;
use super::vertex::{Color, Vector3, UV};
use super::{Mesh, Rgba};
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

pub fn clear(color: &Rgba) {
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

    let mut total_size = (object.vertex.primitives.len()
        * mem::size_of::<Vector3>())
        + (object.vertex.colors.len() * mem::size_of::<Color>());

    object.vertex.uv_coords.iter().for_each(|set| {
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

        // Position data.
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            data_cursor,
            (object.vertex.primitives.len() * mem::size_of::<Vector3>())
                as isize,
            object.vertex.primitives.as_ptr() as *const _,
        );
        data_cursor += (object.vertex.primitives.len()
            * mem::size_of::<Vector3>()) as isize;

        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            data_cursor,
            (object.vertex.colors.len() * mem::size_of::<Color>()) as isize,
            object.vertex.colors.as_ptr() as *const _,
        );

        data_cursor +=
            (object.vertex.colors.len() * mem::size_of::<Color>()) as isize;

        // Texture data.
        object.vertex.uv_coords.iter().for_each(|set| {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                data_cursor,
                (set.coords.len() * mem::size_of::<UV>()) as isize,
                set.coords.as_ptr() as *const _,
            );
            data_cursor += (set.coords.len() * mem::size_of::<UV>()) as isize;
        });

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

    let mut c_type = gl::RGBA;

    // TODO: Why?
    if is_font {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        c_type = gl::RED;
    }

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
        c_type as i32,
        dim.0 as i32,
        dim.1 as i32,
        0,
        c_type,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const c_void,
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);

    tex_id
}

pub fn load_font_to_gpu(
    vertices: &[f32; 24],
    texture: &Texture,
) -> (VAO, VBO, TexId) {
    let vao = gen_vao();
    use_vao(vao);

    unsafe {
        let tex_id = load_tex_to_gpu(vao, texture, true);

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
    object: &Mesh,
) -> (VAO, VBO, Option<EBO>, Vec<TexId>) {
    unsafe {
        let vao = gen_vao();
        let (vbo, ebo) = load_bytes_to_gpu(vao, &object);

        let tex_ids = object
            .textures
            .iter()
            .map(|tex| load_tex_to_gpu(vao, &tex, false))
            .collect();

        use_vao(vao);

        let mut location = 0;
        let mut data_cursor = 0;

        // Positions
        gl::VertexAttribPointer(
            location,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vector3>() as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        location += 1;
        data_cursor +=
            object.vertex.primitives.len() * mem::size_of::<Vector3>();

        // Colors
        gl::VertexAttribPointer(
            location,
            4,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Color>() as i32,
            data_cursor as *const _,
        );
        gl::EnableVertexAttribArray(1);

        location += 1;
        data_cursor += object.vertex.colors.len() * mem::size_of::<Color>();

        // UV Coords.
        for set in object.vertex.uv_coords.iter() {
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
