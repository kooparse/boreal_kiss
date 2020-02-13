pub const VERTEX_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec4 vertex;
    layout (std140) uniform;

    uniform Projections {
        mat4 gui;
        mat4 _;
        mat4 _;
    };

    uniform mat4 model;

    void main() {
        gl_Position = gui * model * vec4(
            vertex.xy, 0.0, 1.0);
    } 
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform vec3 bg_c;

    void main() {    
        FragColor = vec4(bg_c, 1.0);
    }  
"#;
