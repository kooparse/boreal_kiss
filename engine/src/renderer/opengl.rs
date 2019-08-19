use crate::renderer::Color;
use gl;

/// Used to check if opengl is loaded.
pub fn get_opengl_loaded() -> u32 {
    gl::VERSION
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

pub fn clear_color(color: Color) {
    unsafe {
        gl::ClearColor(color.0, color.1, color.2, color.3);
    }
}
