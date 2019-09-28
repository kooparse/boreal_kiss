use super::{DrawMode, Mesh};
use crate::ray::Ray;
use crate::shaders::ShaderType;
use crate::texture::Texture;
use crate::vertex::Vertex;
use gltf;
use nalgebra_glm as glm;

/// Create a renderable triangle object, ready
/// to be consumed by our renderer.
pub fn create_plane<'t, 'n>(
    name: &'n str,
    texture_path: &'t str,
    world_pos: glm::TVec3<f32>,
    scale: f32,
) -> Mesh<'n> {
    let texture = if !texture_path.is_empty() {
        Some(Texture::from_file(texture_path))
    } else {
        None
    };

    let vertex = Vertex {
        primitives: vec![
            glm::vec3(-scale, 0., -scale),
            glm::vec3(-scale, 0., scale),
            glm::vec3(scale, 0., scale),
            glm::vec3(scale, 0., -scale),
        ],
        normals: vec![],
        uv_coords: vec![
            glm::vec2(0.0, 0.0),
            glm::vec2(1.0, 0.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(0.0, 1.0),
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
    };

    let shader_type = texture.as_ref().map_or(ShaderType::SimpleShader, |_| {
        ShaderType::SimpleTextureShader
    });

    Mesh {
        name,
        vertex,
        world_pos,
        texture,
        shader_type,
        mode: DrawMode::Triangles,
    }
}

pub fn load_mesh<'n>(
    path: &'n str,
    world_pos: glm::TVec3<f32>,
    scale: f32,
) -> Mesh<'n> {
    let (model, buffers, images) = gltf::import(path).unwrap();

    let mut loaded_textures: Vec<Texture> = images
        .into_iter()
        .map(|img| Texture::new((img.width, img.height), img.pixels))
        .collect::<_>();

    let mut vertices: Vec<Vertex> = vec![];

    model.meshes().for_each(|mesh| {
        mesh.primitives().for_each(|prim| {
            let mut vertex = Vertex::default();
            let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

            vertex.primitives = {
                reader
                    .read_positions()
                    .unwrap()
                    .map(|pos| {
                        glm::vec3(
                            pos[0] * scale,
                            pos[1] * scale,
                            pos[2] * scale,
                        )
                    })
                    .collect::<_>()
            };

            vertex.indices = {
                reader.read_indices().map_or(vec![], |read_indices| {
                    read_indices.into_u32().collect::<Vec<u32>>()
                })
            };

            if let Some(coords) = reader.read_tex_coords(0) {
                vertex.uv_coords = coords
                    .into_f32()
                    .map(|uv| glm::vec2(uv[0], uv[1]))
                    .collect::<Vec<_>>();
            }

            vertices.push(vertex);
        })
    });

    Mesh {
        name: path,
        vertex: vertices.remove(0),
        world_pos,
        texture: Some(loaded_textures.remove(0)),
        shader_type: ShaderType::SimpleTextureShader,
        mode: DrawMode::Triangles,
    }
}

pub fn create_line<'n>(name: &'n str, ray: Ray) -> Mesh<'n> {
    let vertex = Vertex {
        primitives: vec![ray.origin, ray.direction * ray.length],
        normals: vec![],
        uv_coords: vec![],
        indices: vec![],
    };

    Mesh {
        name,
        vertex,
        world_pos: glm::vec3(0.0, 0.0, 0.0),
        texture: None,
        shader_type: ShaderType::SimpleShader,
        mode: DrawMode::Lines,
    }
}
