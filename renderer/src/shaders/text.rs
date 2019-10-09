use super::{create_shader_program, ShaderProgram, ShaderType};

pub const VERTEX_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec4 vertex;
    out vec2 TexCoords;

    uniform mat4 projection;
    uniform mat4 model;
    uniform float font_size;

    void main() {
        gl_Position = projection * model * vec4(
            vertex.xy * font_size, 0.0, 1.0);

        TexCoords = vertex.zw;
    } 
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core
    in vec2 TexCoords;

    out vec4 FragColor;

    uniform sampler2D texture0;
    uniform vec3 text_color;
    uniform float font_size;

    const float width = 0.5;
    const float edge = 0.1;

    void main() {    

        // float sample = texture(texture0, TexCoords).r;
        // float scale = 1.0; 
        // float signedDistance = (sample - 0.5) * scale;

        // float color = clamp(signedDistance + 0.5, 0.0, 1.0);
        // float alpha = clamp(signedDistance + 0.5 + scale * 0.125, 0.0, 1.0);

        float signed_distance = texture(texture0, TexCoords).r;
        float alpha = smoothstep(width, width + edge, signed_distance);
        FragColor = vec4(text_color, alpha);
    }  
"#;

pub const TYPE: ShaderType = ShaderType::TextShader;

pub fn get_program() -> ShaderProgram {
    ShaderProgram {
        program_id: create_shader_program(VERTEX_SOURCE, FRAGMENT_SOURCE, ""),
    }
}
