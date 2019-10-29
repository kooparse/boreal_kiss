use crate::colliders::{Collider, BoundingBox};
use super::mesh::{Transform, UVSet, Vertex};
use super::shaders::ShaderType;
use super::texture::Texture;
use super::types::{Rgb, Rgba};
use super::SunLight;
use super::Vector;
use super::{DrawMode, Mesh};
use nalgebra_glm as glm;

pub fn create_cube<'n>(position: Vector) -> Mesh<'n> {
    let name = "bbox";

    let vertex = Vertex {
        primitives: vec![
            Vector(-1.0, -1.0, 1.0),
            Vector(1.0, -1.0, 1.0),
            Vector(1.0, 1.0, 1.0),
            Vector(-1.0, 1.0, 1.0),
            // back
            Vector(-1.0, -1.0, -1.0),
            Vector(1.0, -1.0, -1.0),
            Vector(1.0, 1.0, -1.0),
            Vector(-1.0, 1.0, -1.),
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
        ..Default::default()
    };

    let bounding_box = BoundingBox::from_vertex(&vertex);

    Mesh {
        name,
        vertex,
        collider: Some(Collider::Sphere(bounding_box)),
        transform: Transform::from_pos(position),
        textures: vec![],
        shader_type: ShaderType::SimpleShader,
        mode: DrawMode::Triangles,
    }
}

/// Create a renderable triangle object, ready
/// to be consumed by our renderer.
pub fn create_plane<'t, 'n>(
    name: &'n str,
    texture_path: &'t str,
    position: Vector,
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

    let bounding_box = BoundingBox::from_vertex(&vertex);

    Mesh {
        name,
        vertex,
        collider: Some(Collider::Plane(bounding_box)),
        transform: Transform::from_pos(position),
        textures,
        shader_type,
        mode: DrawMode::Triangles,
    }
}

pub fn create_line<'n>(
    name: &'n str,
    origin: Vector,
    dir_vector: Vector,
    color: Rgb,
) -> Mesh<'n> {
    let color = Rgba::from(&color);

    let vertex = Vertex {
        primitives: vec![origin, dir_vector],
        colors: vec![color, color],
        ..Default::default()
    };

    Mesh {
        name,
        vertex,
        collider: None,
        transform: Transform::default(),
        textures: vec![],
        shader_type: ShaderType::SimpleShader,
        mode: DrawMode::Lines,
    }
}

pub fn add_light() -> SunLight {
    SunLight::new(
        Vector(2., 2., 2.),
        Vector(0., 0., 0.),
        Rgba::new(1., 1., 1., 1.),
    )
}

pub fn create_grid<'n>(name: &'n str, position: Vector, dim: i32) -> Mesh<'n> {
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

    Mesh {
        name,
        vertex,
        collider: None,
        transform: Transform::from_pos(position),
        textures: vec![],
        shader_type: ShaderType::SimpleShader,
        mode: DrawMode::Lines,
    }
}
