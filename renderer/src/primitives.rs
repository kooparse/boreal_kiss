use super::{DrawType, Mesh, ShaderType, Texture, Vertex};
use nalgebra_glm as glm;

/// Create a renderable triangle object, ready
/// to be consumed by our renderer.
#[allow(unused)]
pub fn create_triangle_object<'t, 'n>(
    name: &'n str,
    texture_path: &'t str,
    position: glm::TVec3<f32>,
    scale: f32,
) -> Mesh<'t, 'n> {
    let mut tex = Texture::new(texture_path);

    let vertex = Vertex {
        primitives: vec![
            glm::vec3(-scale, -scale, 0.0),
            glm::vec3(-scale, scale, 0.0),
            glm::vec3(scale, scale, 0.0),
            glm::vec3(scale, -scale, 0.0),
        ],
        normals: vec![],
        uv_coords: vec![
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(1.0, 0.0, 0.0),
            glm::vec3(1.0, 1.1, 0.0),
            glm::vec3(0.0, 1.0, 0.0),
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
    };

    Mesh {
        name,
        vertex,
        position,
        texture: Some(tex),
        shader_type: ShaderType::SimpleTextureShader,
        draw_type: DrawType::Triangles,
    }
}

pub fn create_line<'n>(
    name: &'n str,
    position: glm::TVec3<f32>,
) -> Mesh<'_, 'n> {
    let vertex = Vertex {
        primitives: vec![glm::vec3(0., 0., -3.), glm::vec3(0., 0., 3.)],
        normals: vec![],
        uv_coords: vec![],
        indices: vec![],
    };

    Mesh {
        name,
        vertex,
        position,
        texture: None,
        shader_type: ShaderType::SimpleShader,
        draw_type: DrawType::Lines,
    }
}
