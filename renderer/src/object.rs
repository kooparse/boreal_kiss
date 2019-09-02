use super::shaders::ShaderType;
use super::texture::{Texture, UV};

pub struct RendererObject<'a> {
    pub vertices: Vertices,
    pub texture: Option<Texture<'a>>,
    pub shader_type: ShaderType,
    pub gpu_loaded: bool,
}

impl<'a> RendererObject<'a> {
    /// Used when data is push to the gpu.
    /// TODO: Remove this.
    pub fn align(&self) -> Vec<f32> {
        let coords: Option<&Vec<UV>> =
            self.texture.as_ref().and_then(|t| Some(&t.uv.0));

        self.vertices.data.iter().enumerate().fold(
            vec![],
            |mut acc, (i, vertex)| {
                if coords.is_some() {
                    let uv = &coords.unwrap()[i];
                    acc.extend_from_slice(&[
                        vertex.x, vertex.y, vertex.z, uv.x, uv.y, uv.z,
                    ]);
                } else {
                    acc.extend_from_slice(&[vertex.x, vertex.y, vertex.z]);
                }

                acc
            },
        )
    }
}

/// A Vertex is point in 3D model space.
#[allow(unused)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Vertices is the plural of Vertex.
pub struct Vertices {
    pub data: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub ebo: Option<u32>,
}
