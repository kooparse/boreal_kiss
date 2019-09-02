use super::object::{RendererObject, Vertex, Vertices};
use super::shaders::ShaderType;
use super::texture::{TexCoords, Texture, UV};

/// Create a renderable triangle object, ready
/// to be consumed by our renderer.
#[allow(unused)]
pub fn create_triangle_object(
    texture_path: &str,
    scale: f32,
) -> RendererObject {
    let coords = TexCoords(vec![
        UV {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        UV {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        UV {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        UV {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    ]);

    let mut tex = Texture::new(texture_path, coords);

    let vertices = Vertices {
        data: vec![
            Vertex {
                x: -scale,
                y: -scale,
                z: 0.0,
            },
            Vertex {
                x: -scale,
                y: scale,
                z: 0.0,
            },
            Vertex {
                x: scale,
                y: scale,
                z: 0.0,
            },
            Vertex {
                x: scale,
                y: -scale,
                z: 0.0,
            },
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        ebo: None,
    };

    RendererObject {
        vertices,
        gpu_loaded: false,
        texture: Some(tex),
        shader_type: ShaderType::SimpleTextureShader,
    }
}
