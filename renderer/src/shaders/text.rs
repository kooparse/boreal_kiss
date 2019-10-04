use super::{create_shader_program, ShaderProgram, ShaderType};

pub const VERTEX_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec4 vertex;
    out vec2 TexCoords;

    uniform mat4 projection;
    uniform mat4 model;

    void main() {
        gl_Position = projection * model * vec4(vertex.xy, 0.0, 1.0);
        TexCoords = vertex.zw;
    } 
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core
    in vec2 TexCoords;
    out vec4 color;

    uniform sampler2D texture0;
    uniform vec3 text_color;

    void main() {    
        vec4 sampled = vec4(1.0, 1.0, 1.0, texture(texture0, TexCoords).r);
        color = vec4(text_color, 1.0) * sampled;
    }  
"#;

pub const TYPE: ShaderType = ShaderType::TextShader;

pub fn get_program() -> ShaderProgram {
    ShaderProgram {
        program_id: create_shader_program(VERTEX_SOURCE, FRAGMENT_SOURCE, ""),
    }
}
