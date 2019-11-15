use super::mesh::{UVSet, Vertex};
use super::shaders::ShaderType;
use super::texture::Texture;
use super::types::Rgba;
use super::SunLight;
use super::Vector;
use super::{DrawMode, Mesh, Transform};
use crate::colliders::Collider;
use crate::entities::Handle;
use nalgebra_glm as glm;

pub fn create_cube<'n>(
    transform: Transform,
    parent: Option<Handle<Mesh>>,
    color: Rgba,
) -> Mesh {
    let x_scalar = 1.;
    let y_scalar = 1.;
    let z_scalar = 1.;

    let vertex = Vertex {
        primitives: vec![
            Vector(-x_scalar, -y_scalar, z_scalar),
            Vector(x_scalar, -y_scalar, z_scalar),
            Vector(x_scalar, y_scalar, z_scalar),
            Vector(-x_scalar, y_scalar, z_scalar),
            // back
            Vector(-x_scalar, -y_scalar, -z_scalar),
            Vector(x_scalar, -y_scalar, -z_scalar),
            Vector(x_scalar, y_scalar, -z_scalar),
            Vector(-x_scalar, y_scalar, -z_scalar),
        ],
        indices: vec![
            // front
            0, 1, 2, 2, 3, 0, // right
            1, 5, 6, 6, 2, 1, // back
            7, 6, 5, 5, 4, 7, // left
            4, 0, 3, 3, 7, 4, // bottom
            4, 5, 1, 1, 0, 4, // top
            3, 2, 6, 6, 7, 3,
        ],
        colors: vec![color, color, color, color, color, color, color, color],
        ..Default::default()
    };

    Mesh::new(
        vertex,
        vec![],
        transform,
        parent,
        Some(Collider::Cube),
        DrawMode::Triangles,
        ShaderType::SimpleShader,
    )
}

/// Create a renderable triangle object, ready
/// to be consumed by our renderer.
pub fn create_plane<'t>(texture_path: &'t str, transform: Transform) -> Mesh {
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
            Vector(-1., 0., -1.),
            Vector(-1., 0., 1.),
            Vector(1., 0., 1.),
            Vector(1., 0., -1.),
        ],
        uv_coords,
        indices: vec![0, 1, 2, 0, 2, 3],
        ..Default::default()
    };

    Mesh::new(
        vertex,
        textures,
        transform,
        None,
        Some(Collider::Cube),
        DrawMode::Triangles,
        ShaderType::SimpleShader,
    )
}

// pub fn create_line<'n>(
//     name: &'n str,
//     origin: Vector,
//     dir_vector: Vector,
//     color: Rgb,
// ) -> Mesh {
//     let color = Rgba::from(&color);

//     let vertex = Vertex {
//         primitives: vec![origin, dir_vector],
//         colors: vec![color, color],
//         ..Default::default()
//     };

//     Mesh::new(
//         vertex,
//         vec![],
//         transform,
//         None,
//         None,
//         DrawMode::Lines,
//         ShaderType::SimpleShader,
//     )
// }

pub fn add_light() -> SunLight {
    SunLight::new(
        Vector(2., 2., 2.),
        Vector(0., 0., 0.),
        Rgba::new(1., 1., 1., 1.),
    )
}

pub fn create_grid(transform: Transform, dim: i32) -> Mesh {
    let scale = 5f32;
    let mut list: Vec<Vector> = vec![];
    let mut colors: Vec<Rgba> = vec![];
    let ratio = (dim / 2) as f32;

    let purple = Rgba::new(1., 0., 0.5, 1.);

    for i in 0..=dim {
        // Rows
        let r = (i as f32 / ratio) * scale;
        list.extend_from_slice(&[
            Vector(-scale, 0., -scale + r),
            Vector(scale, 0., -scale + r),
        ]);
        colors.extend_from_slice(&[purple, purple]);

        for j in 0..=dim {
            // Columns
            let c = (j as f32 / ratio) * scale;
            list.extend_from_slice(&[
                Vector(-scale + c, 0., scale),
                Vector(-scale + c, 0., -scale),
            ]);
            colors.extend_from_slice(&[purple, purple]);
        }
    }

    let vertex = Vertex {
        primitives: list,
        colors,
        ..Default::default()
    };

    Mesh::new(
        vertex,
        vec![],
        transform,
        None,
        None,
        DrawMode::Lines,
        ShaderType::SimpleShader,
    )
}
