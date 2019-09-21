use super::Ray;
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
            glm::vec3(-scale, 0., -scale),
            glm::vec3(-scale, 0., scale),
            glm::vec3(scale, 0., scale),
            glm::vec3(scale, 0., -scale),
        ],
        normals: vec![],
        uv_coords: vec![
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(1.0, 0.0, 0.0),
            glm::vec3(1.0, 0.0, 1.0),
            glm::vec3(0.0, 0.0, 1.0),
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

pub fn create_line<'n>(name: &'n str, ray: Ray) -> Mesh<'_, 'n> {
    let vertex = Vertex {
        primitives: vec![ray.origin, ray.direction * ray.length],
        normals: vec![],
        uv_coords: vec![],
        indices: vec![],
    };

    Mesh {
        name,
        vertex,
        position: ray.origin,
        texture: None,
        shader_type: ShaderType::SimpleShader,
        draw_type: DrawType::Lines,
    }
}
