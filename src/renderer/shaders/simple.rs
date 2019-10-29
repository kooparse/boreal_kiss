pub const VERTEX_SOURCE: &str = r#"
    #version 330 core
    layout (std140) uniform;

    layout (location = 0) in vec3 a_pos;
    layout (location = 1) in vec4 a_color;
    layout (location = 2) in vec2 a_uv_coords[2];

    uniform Projections {
        mat4 gui;
        mat4 perspective;
        mat4 view;
    };

    uniform mat4 model;

    out VERTEX_OUT {
        vec4 color;
        vec2 uv_coords[2];
    } vs_out;

    void main() {
       	gl_Position = perspective * view * model * vec4(a_pos, 1.0);
        vs_out.color = a_color;
	vs_out.uv_coords = a_uv_coords;
    }
"#;

pub const FRAGMENT_SOURCE: &str = r#"
    #version 330 core

    uniform bool HAS_UV;
    uniform bool HAS_MULTI_UV;
    uniform bool HAS_VERT_COLORS;

    uniform sampler2D texture0;
    uniform sampler2D texture1;

    uniform bool is_active;
    uniform bool is_hover;

    in VERTEX_OUT {
        vec4 color;
        vec2 uv_coords[2];
    } vertex_in;


    out vec4 FragColor;

    void main() {
        vec4 color = vec4(1.0, 1.0, 1.0, 1.0);

        if (HAS_VERT_COLORS) {
            color = vertex_in.color;
        }

        if (HAS_UV) {
            color = texture(texture0, vertex_in.uv_coords[0]);
        }

        if (HAS_MULTI_UV) {
            color = texture(texture0, vertex_in.uv_coords[0]) 
                + texture(texture1, vertex_in.uv_coords[1]);
        }

        if (is_hover) {
            color = vec4(color.xyz, 0.7);
        }


        FragColor = color;
    }
"#;
