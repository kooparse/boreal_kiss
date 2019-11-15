pub const VERTEX_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec4 vertex;
    layout (std140) uniform;
    out vec2 TexCoords;

    uniform Projections {
        mat4 gui;
        mat4 _;
        mat4 _;
    };

    uniform mat4 model;
    uniform float font_size;

    void main() {
        gl_Position = gui * model * vec4(
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

    float width = 0.51; 
    float edge = 0.045;

    void main() {    
        float distance = texture(texture0, TexCoords).r;
        float alpha = smoothstep(width - edge , width + edge, distance);

        FragColor = vec4(text_color, alpha);
    }  
"#;
