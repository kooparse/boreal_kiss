use nalgebra_glm as glm;

pub type Vector3 = glm::TVec3<f32>;
pub type UV = glm::TVec2<f32>;

#[derive(Debug, Default)]
pub struct Vertex {
    pub primitives: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uv_coords: Vec<UV>,
    pub indices: Vec<u32>,
}
