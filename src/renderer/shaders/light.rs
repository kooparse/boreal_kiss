pub const VERTEX_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 local_pos;

    uniform mat4 projection;
    uniform mat4 model;
    uniform mat4 view;

    void main() {
       	gl_Position = projection * view * model * vec4(local_pos, 1.0);
    } 
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core

    out vec4 FragColor;

    uniform vec3 entity_color;
    uniform vec3 light_color;

    void main() {    
        float ambient_strength = 0.1;
        vec3 ambient = ambient_strength * light_color;

        vec3 result = ambient * entity_color;
        FragColor = vec4(result, 1.0);
    }  
"#;
