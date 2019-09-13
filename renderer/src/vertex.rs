use nalgebra_glm as glm;

pub type Vector3 = glm::TVec3<f32>;

#[derive(Debug)]
pub struct Vertex {
    pub primitives: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uv_coords: Vec<Vector3>,
    pub indices: Vec<u32>,
}
