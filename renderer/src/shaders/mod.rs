pub mod simple;
pub mod simple_with_tex;
use super::opengl;
use gl::{self, types::GLchar};
use std::{collections::HashMap, ffi::CString, ptr, str};

pub type ShaderProgramId = u32;

// All shaders available.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ShaderType {
    SimpleShader,
    SimpleTextureShader,
}

/// Map vertex array object to shader.
pub struct ShaderProgram {
    pub program_id: ShaderProgramId,
    pub vao: u32,
}

/// We use one vao per shader; I guess that it's the
/// good approach for now.
pub struct ShaderManager {
    pub list: HashMap<ShaderType, ShaderProgram>,
}

impl ShaderManager {
    pub fn new() -> Self {
        let mut list = HashMap::new();

        list.insert(simple::TYPE, simple::get_program());
        list.insert(simple_with_tex::TYPE, simple_with_tex::get_program());

        Self { list }
    }
}

/// Delete all shader programs when shader manager is drop.
impl Drop for ShaderManager {
    fn drop(&mut self) {
        for program in self.list.values() {
            unsafe { gl::DeleteProgram(program.program_id) }
        }
    }
}

pub fn set_matrix4(
    shader_id: ShaderProgramId,
    var_name: &str,
    transform: &[f32],
) {
    let shader_variable = get_location(shader_id, var_name);
    unsafe {
        gl::UniformMatrix4fv(shader_variable, 1, gl::FALSE, transform.as_ptr());
    }
}

fn get_location(shader_id: ShaderProgramId, var_name: &str) -> i32 {
    let var_name = CString::new(var_name)
        .expect("Crash while converting Rust str to C string");
    unsafe { gl::GetUniformLocation(shader_id, var_name.as_ptr()) }
}

pub fn create_shader_program(
    vertex_source: &str,
    fragment_source: &str,
    geometry_source: &str,
) -> ShaderProgramId {
    unsafe {
        let shader_program = gl::CreateProgram();

        // For vertex shader.
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertex_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        gl::AttachShader(shader_program, vertex_shader);

        // For fragment shader.
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_vert = CString::new(fragment_source.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        gl::AttachShader(shader_program, fragment_shader);

        if !geometry_source.is_empty() {
            // For geometry shader.
            let geometry_shader = gl::CreateShader(gl::GEOMETRY_SHADER);
            let c_str_vert = CString::new(geometry_source.as_bytes()).unwrap();
            gl::ShaderSource(
                geometry_shader,
                1,
                &c_str_vert.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(fragment_shader);
            gl::AttachShader(shader_program, geometry_shader);
        }

        // Link all shaders
        gl::LinkProgram(shader_program);

        // Check for linking errors
        let mut is_success = 0;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut is_success);
        if is_success != 1 {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            dbg!("Error while compiling shader program:");
            dbg!(str::from_utf8(&info_log).unwrap());
        }

        // Clear our shader.
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}
