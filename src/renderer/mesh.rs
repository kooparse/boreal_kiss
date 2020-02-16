use super::opengl;
use super::shaders::{ShaderFlags, ShaderType};
use super::texture::Texture;
use super::DrawMode;
use super::GpuBound;
use super::Vector;
use crate::colliders::{BoundingBox, Collider};
use crate::entities::Handle;
use std::cmp::min;

use super::types::Rgba;
use gltf;
use nalgebra_glm as glm;

pub type Vector3 = Vector;
pub type UV = glm::TVec2<f32>;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vector,
    pub rotation: Vector,
    pub scale: Vector,
}

impl Transform {
    pub fn scale(mut self, scale: Vector) -> Self {
        self.scale = scale;
        self
    }

    pub fn from_pos(position: Vector) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn to_glm(&self) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
        (
            self.position.to_glm(),
            self.rotation.to_glm(),
            self.scale.to_glm(),
        )
    }

    pub fn to_model(&self) -> glm::Mat4 {
        let (pos, rotation, scale) = self.to_glm();
        let identity = glm::identity();

        let scale_matrix = glm::scale(&identity, &scale);
        let translate_matrix = glm::translate(&identity, &pos);

        let rotation_matrix = {
            let rot_x_matrix = glm::rotate_x(&identity, rotation.x);
            let rot_y_matrix = glm::rotate_y(&identity, rotation.y);
            let rot_z_matrix = glm::rotate_z(&identity, rotation.z);

            rot_x_matrix * rot_y_matrix * rot_z_matrix
        };

        translate_matrix * rotation_matrix * scale_matrix
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector::default(),
            rotation: Vector::default(),
            scale: Vector(1., 1., 1.),
        }
    }
}

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
pub struct Mesh {
    // Debug from editor...
    pub is_hover: bool,
    pub is_selected: bool,
    pub is_dragged: bool,
    pub is_hidden: bool,

    pub(crate) gpu_bound: GpuBound,
    pub flags: ShaderFlags,

    pub transform: Transform,
    pub parent: Option<Handle<Mesh>>,

    pub bounding_box: BoundingBox,

    pub vertex: Vertex,
    pub textures: Vec<Texture>,
    pub shader_type: ShaderType,
    pub mode: DrawMode,
    pub collider: Option<Collider>,
}

impl Mesh {
    pub fn new(
        vertex: Vertex,
        textures: Vec<Texture>,
        transform: Transform,
        parent: Option<Handle<Mesh>>,
        collider: Option<Collider>,
        mode: DrawMode,
        shader_type: ShaderType,
    ) -> Self {
        let (gpu_bound, flags) = Self::load_gl(&vertex, &textures, shader_type);
        let bounding_box = BoundingBox::from_vertex(&vertex);

        Self {
            bounding_box,
            collider,
            vertex,
            textures,
            parent,
            shader_type,
            mode,
            transform,

            gpu_bound,
            flags,

            is_hover: false,
            is_selected: false,
            is_dragged: false,
            is_hidden: false,
        }
    }

    pub fn load_gl(
        vertex: &Vertex,
        textures: &Vec<Texture>,
        shader_type: ShaderType,
    ) -> (GpuBound, ShaderFlags) {
        // From system memmory to gpu memory.
        let (vao, vbo, ebo, tex_ids) =
            opengl::load_object_to_gpu(vertex, textures);

        let primitives_len =
            ebo.map_or(vertex.primitives.len(), |_| vertex.indices.len());

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo,
            primitives_len,
            shader: shader_type,
            tex_ids,
        };

        let (has_uv, has_multi_uv, has_vert_colors, _tex_number) = {
            let colors = &vertex.colors;
            // We want a correlation between the number of set of coords
            // and the number of texture loaded.
            let tex_number = min(vertex.uv_coords.len(), textures.len());

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

        (gpu_bound, flags)
    }

    pub fn from_gltf(path: &str, transform: Transform) -> Vec<Mesh> {
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
                        .map(|pos| Vector(pos[0], pos[1], pos[2]))
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

        // TODO: Careful here...
        // let bounding_box =
        //     BoundingBox::from_vertex(&vertices[0], &transform.scale);

        vertices
            .into_iter()
            .map(|v| {
                Mesh::new(
                    v,
                    textures.clone(),
                    transform,
                    None,
                    // collider
                    None,
                    DrawMode::Triangles,
                    ShaderType::SimpleShader,
                )
            })
            .collect()
    }
}
