use super::{create_shader_program, ShaderProgram, ShaderType};

pub const VERTEX_SOURCE: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;
    layout (location = 2) in vec2 aTexCoord2;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec2 TexCoord;
    out vec2 TexCoord2;

    void main() {
       	gl_Position = projection * view * model * vec4(aPos.xyz, 1.0);
	TexCoord = aTexCoord;
	TexCoord2 = aTexCoord2;
    }
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core

    in vec2 TexCoord;
    in vec2 TexCoord2;

    uniform bool HAS_UV;
    uniform bool HAS_MULTI_UV;

    out vec4 FragColor;

    uniform sampler2D texture0;
    uniform sampler2D texture1;

    void main() {
        vec4 color = vec4(1.0, 1.0, 1.0, 1.0);

        if (HAS_UV) {
            color = texture(texture0, TexCoord);
        }

        if (HAS_MULTI_UV) {
            color = 
                texture(texture0, TexCoord) +
                texture(texture1, TexCoord2);
        }

        FragColor = color;
    }
"#;

pub const TYPE: ShaderType = ShaderType::SimpleTextureShader;

pub fn get_program() -> ShaderProgram {
    ShaderProgram {
        program_id: create_shader_program(VERTEX_SOURCE, FRAGMENT_SOURCE, ""),
    }
}
