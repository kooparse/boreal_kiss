use super::{create_shader_program, ShaderProgram, ShaderType};

pub const VERTEX_SOURCE: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aTexCoord;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 TexCoord;

    void main() {
       	gl_Position = projection * view * model * vec4(aPos.xyz, 1.0);
	TexCoord = aTexCoord;
    }
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core

    in vec3 TexCoord;
    out vec4 FragColor;

    uniform sampler2D tex_sample;

    void main() {
       	FragColor = texture(tex_sample, TexCoord.xz);
    }
"#;

pub const TYPE: ShaderType = ShaderType::SimpleTextureShader;

pub fn get_program() -> ShaderProgram {
    ShaderProgram {
        program_id: create_shader_program(VERTEX_SOURCE, FRAGMENT_SOURCE, ""),
    }
}
