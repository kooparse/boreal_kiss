use nalgebra_glm as glm;
use renderer::{primitives, Mesh};

pub fn scene_mesh() -> Vec<Mesh<'static>> {
    vec![
        primitives::create_grid("debug_grid", glm::vec3(0., 0.0, 0.0), 20),
        primitives::create_plane(
            "plane_1",
            "assets/textures/pos_debug.png",
            glm::vec3(0., 0.0, 0.0),
            1.0,
        ),
        primitives::load_mesh(
            "assets/models/cube/Cube.gltf",
            glm::vec3(2., 0.0, 0.0),
            0.7,
        ),
        primitives::load_mesh(
            "assets/models/cube_color/BoxVertexColors.gltf",
            glm::vec3(-2., 0.0, 0.0),
            0.7,
        ),
        primitives::load_mesh(
            "assets/models/cube_tex/BoxTextured.gltf",
            glm::vec3(0., -2., 0.0),
            1.,
        ),
        primitives::load_mesh(
            "assets/models/multi_uv/MultiUVTest.gltf",
            glm::vec3(0., 2., 0.0),
            1.,
        ),
    ]
}

pub fn scene_light() -> Vec<Mesh<'static>> {
    vec![
        primitives::create_grid("debug_grid", glm::vec3(0., 0.0, 0.0), 20),
        primitives::create_plane(
            "plane_1",
            "assets/textures/pos_debug.png",
            glm::vec3(0., 1.2, 0.0),
            1.0,
        ),
        primitives::load_mesh(
            "assets/models/cube/Cube.gltf",
            glm::vec3(0., 0.0, 0.0),
            0.7,
        ),
        primitives::load_mesh(
            "assets/models/cube_tex/BoxTextured.gltf",
            glm::vec3(0., 2., 0.0),
            1.,
        ),
        primitives::load_mesh(
            "assets/models/multi_uv/MultiUVTest.gltf",
            glm::vec3(0., -2., 0.0),
            1.,
        ),
    ]
}
