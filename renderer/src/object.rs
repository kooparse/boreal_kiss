use super::opengl;
use super::shaders::ShaderType;
use super::texture::Texture;
use super::{GpuBound, LoadedObject};
use nalgebra_glm as glm;

pub struct RendererObject<'a> {
    pub vertices: Vertices,
    pub texture: Option<Texture<'a>>,
    pub shader_type: ShaderType,
    pub gpu_loaded: bool,
    pub position: glm::TVec3<f32>,
}

impl<'a> From<RendererObject<'a>> for LoadedObject {
    fn from(object: RendererObject<'a>) -> LoadedObject {
        let vao = opengl::gen_vao();
        let vbo = opengl::gen_buffer();

        let ebo = if !object.vertices.indices.is_empty() {
            Some(opengl::gen_buffer())
        } else {
            None
        };

        let texture_id = if let Some(texture) = &object.texture {
            Some(texture.id)
        } else {
            None
        };

        // From system memmory to gpu memory.
        opengl::load_object_to_gpu((vao, vbo, ebo), &object);

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo,
            data_len: if ebo.is_some() {
                object.vertices.indices.len()
            } else {
                object.vertices.data.len()
            },
            shader: object.shader_type,
            texture_id,
        };

        LoadedObject {
            position: object.position,
            gpu_bound,
        }
    }
}

/// A Vertex is point in 3D model space.
#[allow(unused)]
#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Vertices is the plural of Vertex.
pub struct Vertices {
    pub data: Vec<Vertex>,
    pub indices: Vec<u32>,
}
