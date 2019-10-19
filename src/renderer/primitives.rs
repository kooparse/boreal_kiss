use super::SunLight;
use super::Vector;
use super::{DrawMode, Mesh};
// use super::ray::Ray;
use super::mesh::{UVSet, Vertex};
use super::shaders::ShaderType;
use super::texture::Texture;
use super::types::Rgba;
use nalgebra_glm as glm;

/// Create a renderable triangle object, ready
/// to be consumed by our renderer.
pub fn create_plane<'t, 'n>(
    name: &'n str,
    texture_path: &'t str,
    world_pos: Vector,
    scale: f32,
) -> Mesh<'n> {
    let shader_type = ShaderType::SimpleShader;
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
            Vector(-scale, 0., -scale),
            Vector(-scale, 0., scale),
            Vector(scale, 0., scale),
            Vector(scale, 0., -scale),
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

// pub fn create_line<'n>(name: &'n str, ray: Ray) -> Mesh<'n> {
//     let vertex = Vertex {
//         primitives: vec![ray.origin, ray.direction * ray.length],
//         ..Default::default()
//     };

//     Mesh {
//         name,
//         vertex,
//         world_pos: Vector(0.0, 0.0, 0.0),
//         textures: vec![],
//         shader_type: ShaderType::SimpleShader,
//         mode: DrawMode::Lines,
//     }
// }

pub fn add_light() -> SunLight {
    SunLight::new(
        Vector(2., 2., 2.),
        Vector(0., 0., 0.),
        Rgba::new(1., 1., 1., 1.),
    )
}

pub fn create_grid<'n>(name: &'n str, world_pos: Vector, dim: i32) -> Mesh<'n> {
    let scale = 5f32;
    let mut list: Vec<Vector> = vec![];
    let ratio = (dim / 2) as f32;

    for i in 0..=dim {
        // Rows
        let r = (i as f32 / ratio) * scale;
        list.push(Vector(-scale, 0., -scale + r));
        list.push(Vector(scale, 0., -scale + r));

        for j in 0..=dim {
            // Columns
            let c = (j as f32 / ratio) * scale;
            list.push(Vector(-scale + c, 0., scale));
            list.push(Vector(-scale + c, 0., -scale));
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
        shader_type: ShaderType::SimpleShader,
        mode: DrawMode::Lines,
    }
}
