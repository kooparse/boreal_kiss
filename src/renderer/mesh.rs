use super::opengl;
use super::shaders::{ShaderFlags, ShaderType};
use super::texture::Texture;
use super::DrawMode;
use super::GpuBound;
use super::Vector;
use std::cmp::min;

use super::types::Rgba;
use nalgebra_glm as glm;
use gltf;

pub type Vector3 = Vector;
pub type UV = glm::TVec2<f32>;

#[derive(Debug, Default)]
pub struct UVSet {
    set: u32,
    pub coords: Vec<UV>,
}

impl UVSet {
    pub fn new(set: u32, coords: Vec<UV>) -> Self {
        Self { set, coords }
    }
}

#[derive(Debug, Default)]
pub struct Vertex {
    pub primitives: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Rgba>,
    pub uv_coords: Vec<UVSet>,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub struct LoadedMesh {
    pub is_hidden: bool,
    pub world_pos: Vector,
    pub mode: DrawMode,
    pub(crate) gpu_bound: GpuBound,
    pub flags: ShaderFlags,
}

pub struct Mesh<'n> {
    pub name: &'n str,
    pub vertex: Vertex,
    pub textures: Vec<Texture>,
    pub shader_type: ShaderType,
    pub world_pos: Vector,
    pub mode: DrawMode,
}

impl<'n> Mesh<'_> {
    pub fn from_gltf(path: &'n str, world_pos: Vector, scale: f32) -> Mesh<'n> {
        let (model, buffers, images) = gltf::import(path).unwrap();
        let mut vertices: Vec<Vertex> = vec![];

        model.meshes().for_each(|mesh| {
            mesh.primitives().for_each(|prim| {
                let mut vertex = Vertex::default();
                let reader =
                    prim.reader(|buffer| Some(&buffers[buffer.index()]));

                vertex.primitives = {
                    reader
                        .read_positions()
                        .unwrap()
                        .map(|pos| {
                            Vector(
                                pos[0] * scale,
                                pos[1] * scale,
                                pos[2] * scale,
                            )
                        })
                        .collect()
                };

                vertex.colors = {
                    reader.read_colors(0).map_or(vec![], |read_colors| {
                        read_colors
                            .into_rgba_f32()
                            .map(|color| {
                                Rgba::new(
                                    color[0], color[1], color[2], color[3],
                                )
                            })
                            .collect()
                    })
                };

                vertex.indices = {
                    reader.read_indices().map_or(vec![], |read_indices| {
                        read_indices.into_u32().collect()
                    })
                };

                let mut tex_set = 0;
                while let Some(coords) = reader.read_tex_coords(tex_set) {
                    let coords: Vec<UV> = coords
                        .into_f32()
                        .map(|uv| glm::vec2(uv[0], uv[1]))
                        .collect();

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

        Mesh {
            name: path,
            vertex: vertices.remove(0),
            world_pos,
            textures,
            shader_type: ShaderType::SimpleShader,
            mode: DrawMode::Triangles,
        }
    }
}

impl<'n> From<&Mesh<'n>> for LoadedMesh {
    fn from(object: &Mesh<'n>) -> LoadedMesh {
        // From system memmory to gpu memory.
        let (vao, vbo, ebo, tex_ids) = opengl::load_object_to_gpu(&object);

        let primitives_len = ebo.map_or(object.vertex.primitives.len(), |_| {
            object.vertex.indices.len()
        });

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo,
            primitives_len,
            shader: object.shader_type,
            tex_ids,
        };

        let (has_uv, has_multi_uv, has_vert_colors, _tex_number) = {
            let colors = &object.vertex.colors;
            // We want a correlation between the number of set of coords
            // and the number of texture loaded.
            let tex_number =
                min(object.vertex.uv_coords.len(), object.textures.len());

            (
                tex_number > 0,
                tex_number > 1,
                !colors.is_empty(),
                tex_number,
            )
        };

        let flags = ShaderFlags {
            has_uv,
            has_multi_uv,
            has_vert_colors,
        };

        LoadedMesh {
            is_hidden: false,
            mode: object.mode,
            world_pos: object.world_pos,
            gpu_bound,
            flags,
        }
    }
}
