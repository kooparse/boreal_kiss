use super::{create_shader_program, ShaderProgram, ShaderType};

pub const VERTEX_SOURCE: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec4 aRgba;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec4 Rgba;

    void main() {
       	gl_Position = projection * view * model * vec4(aPos.xyz, 1.0);
        Rgba = aRgba;
    }
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core

    in vec4 Rgba;
    out vec4 FragColor;

    void main() {
	FragColor = Rgba;
    }
"#;

pub const TYPE: ShaderType = ShaderType::SimpleShader;

pub fn get_program() -> ShaderProgram {
    ShaderProgram {
        program_id: create_shader_program(VERTEX_SOURCE, FRAGMENT_SOURCE, ""),
    }
}
