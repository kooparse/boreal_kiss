use crate::renderer::{primitives, Mesh, Vector};

pub fn scene_mesh() -> Vec<Mesh<'static>> {
    vec![
        primitives::create_grid("debug_grid", Vector(0., 0.0, 0.0), 20),
        primitives::create_cube(Vector(-3., 0.0, 0.0)),
        primitives::create_plane(
            "plane_1",
            "assets/textures/pos_debug.png",
            Vector(0., 0.0, 0.0),
            1.0,
        ),
        Mesh::from_gltf(
            "assets/models/sphere/sphere.gltf",
            Vector(3., 0.0, 0.0),
            1.,
        ),
        Mesh::from_gltf(
            "assets/models/cube_color/BoxVertexColors.gltf",
            Vector(-2., 0.0, 0.0),
            0.7,
        ),
        Mesh::from_gltf(
            "assets/models/cube_tex/BoxTextured.gltf",
            Vector(0., -2., 0.0),
            1.,
        ),
        Mesh::from_gltf(
            "assets/models/multi_uv/MultiUVTest.gltf",
            Vector(0., 2., 0.0),
            1.,
        ),
    ]
}

pub fn scene_light() -> Vec<Mesh<'static>> {
    vec![
        primitives::create_grid("debug_grid", Vector(0., 0.0, 0.0), 20),
        primitives::create_plane("plane_1", "", Vector(0., 0., 0.), 5.0),
        Mesh::from_gltf(
            "assets/models/cube/Cube.gltf",
            Vector(0., 1.0, 0.0),
            1.,
        ),
    ]
}
