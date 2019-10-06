use nalgebra_glm as glm;
use crate::color::Rgba;

pub type Vector3 = glm::TVec3<f32>;
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
