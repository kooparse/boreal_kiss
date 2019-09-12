use super::{create_shader_program, ShaderProgram, ShaderType};

pub const VERTEX_SOURCE: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aPos;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;


    void main() {
       	gl_Position = projection * view * model * vec4(aPos.xyz, 1.0);
    }
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core

    out vec4 FragColor;

    void main() {
	FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

pub const TYPE: ShaderType = ShaderType::SimpleShader;

pub fn get_program() -> ShaderProgram {
    ShaderProgram {
        program_id: create_shader_program(VERTEX_SOURCE, FRAGMENT_SOURCE, ""),
    }
}
