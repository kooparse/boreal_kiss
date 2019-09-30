use super::{DrawMode, Mesh};
use crate::ray::Ray;
use crate::shaders::ShaderType;
use crate::texture::Texture;
use crate::vertex::{UVSet, Vertex, UV};
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
    let shader_type = ShaderType::SimpleTextureShader;
    let mut textures = vec![];
    let mut uv_coords = vec![];

    if !texture_path.is_empty() {
        uv_coords.push(UVSet::new(
            0,
            vec![
                glm::vec2(0.0, 0.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(1.0, 1.0),
                glm::vec2(0.0, 1.0),
            ],
        ));
        textures.push(Texture::from_file(texture_path));
    };

    let vertex = Vertex {
        primitives: vec![
            glm::vec3(-scale, 0., -scale),
            glm::vec3(-scale, 0., scale),
            glm::vec3(scale, 0., scale),
            glm::vec3(scale, 0., -scale),
        ],
        uv_coords,
        indices: vec![0, 1, 2, 0, 2, 3],
        ..Default::default()
    };

    Mesh {
        name,
        vertex,
        world_pos,
        textures,
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

    let mut shader_type = ShaderType::SimpleShader;
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

            vertex.colors = {
                reader.read_colors(0).map_or(vec![], |read_colors| {
                    read_colors
                        .into_rgba_f32()
                        .map(|color| {
                            glm::vec4(color[0], color[1], color[2], color[3])
                        })
                        .collect::<_>()
                })
            };

            vertex.indices = {
                reader.read_indices().map_or(vec![], |read_indices| {
                    read_indices.into_u32().collect::<Vec<u32>>()
                })
            };

            let mut tex_set = 0;
            while let Some(coords) = reader.read_tex_coords(tex_set) {
                let coords: Vec<UV> = coords
                    .into_f32()
                    .map(|uv| glm::vec2(uv[0], uv[1]))
                    .collect::<Vec<_>>();

                vertex.uv_coords.push(UVSet::new(tex_set, coords));
                tex_set += 1;
            }

            vertices.push(vertex);
        })
    });

    let textures: Vec<Texture> = images
        .into_iter()
        .map(|img| Texture::new((img.width, img.height), img.pixels))
        .collect::<_>();

    if !textures.is_empty() {
        shader_type = ShaderType::SimpleTextureShader;
    }

    Mesh {
        name: path,
        vertex: vertices.remove(0),
        world_pos,
        textures,
        shader_type,
        mode: DrawMode::Triangles,
    }
}

pub fn create_line<'n>(name: &'n str, ray: Ray) -> Mesh<'n> {
    let vertex = Vertex {
        primitives: vec![ray.origin, ray.direction * ray.length],
        ..Default::default()
    };

    Mesh {
        name,
        vertex,
        world_pos: glm::vec3(0.0, 0.0, 0.0),
        textures: vec![],
        shader_type: ShaderType::SimpleTextureShader,
        mode: DrawMode::Lines,
    }
}

pub fn create_grid<'t, 'n>(
    name: &'n str,
    world_pos: glm::TVec3<f32>,
    dim: i32,
) -> Mesh<'n> {
    let scale = 5f32;
    let mut list: Vec<glm::TVec3<f32>> = vec![];

    for i in 0..(dim + 1) {
        for j in 0..(dim + 1) {
            let ratio = (dim / 2) as f32;
            let r = (i as f32 / ratio) * scale;
            let c = (j as f32 / ratio) * scale;

            list.push(glm::vec3(-scale, 0., -scale + r));
            list.push(glm::vec3(scale, 0., -scale + r));

            list.push(glm::vec3(-scale + c, 0., scale));
            list.push(glm::vec3(-scale + c, 0., -scale));
        }
    }

    let vertex = Vertex {
        primitives: list,
        ..Default::default()
    };

    Mesh {
        name,
        vertex,
        world_pos,
        textures: vec![],
        shader_type: ShaderType::SimpleTextureShader,
        mode: DrawMode::Lines,
    }
}
