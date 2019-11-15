use crate::entities::Entities;
use crate::renderer::{primitives, Mesh, Rgba, Transform, Vector};

pub fn scene_mesh(entities: &mut Entities) -> Vec<Mesh> {
    let position = Vector(0., 0., 0.);

    let parent = Transform::from_pos(position);

    let lenght = 0.5;
    let weight = 0.5;

    let mut transform = Transform::from_pos(position);
    transform.scale = Vector(lenght, weight, weight);
    transform.rotation = Vector(0., 0., 0.);
    // transform.position = Vector(-lenght, weight, 0.);
    let red = Rgba::new(1., 0., 0., 1.);
    let axis_x = primitives::create_cube(transform, None, red);

    let mut transform = Transform::from_pos(position);
    transform.scale = Vector(lenght, weight, weight);
    transform.rotation = Vector(0., 0., 90f32.to_radians());
    transform.position = Vector(0., lenght, 0.);
    let green = Rgba::new(0., 1., 0., 1.);
    let axis_y = primitives::create_cube(transform, None, green);

    let mut transform = Transform::from_pos(position);
    transform.scale = Vector(lenght, weight, weight);
    transform.rotation = Vector(0., 90f32.to_radians(), 0.);
    // transform.position = Vector(0., weight, lenght);
    let blue = Rgba::new(0., 0., 1., 1.);
    let axis_z = primitives::create_cube(transform, None, blue);

    vec![
        primitives::create_grid(Transform::default(), 20),
        // axis_x,
        axis_y,
        // axis_z,
        // primitives::create_plane(
        //     "assets/textures/pos_debug.png",
        //     entities.transforms.insert(Transform::default())
        // ),
    ]

    // vec![
    //     primitives::create_grid("debug_grid", Vector(0., 0.0, 0.0), 20),
    //     primitives::create_cube(Vector(-3., 0.0, 0.0)),
    //     primitives::create_plane(
    //         "plane_1",
    //         "assets/textures/pos_debug.png",
    //         Vector(0., 0.0, 0.0),
    //         1.0,
    //     ),
    //     Mesh::from_gltf(
    //         "assets/models/sphere/sphere.gltf",
    //         Vector(3., 0.0, 0.0),
    //         1.,
    //     ),
    //     // Mesh::from_gltf(
    //     //     "assets/models/cube_color/BoxVertexColors.gltf",
    //     //     Vector(-2., 0.0, 0.0),
    //     //     0.7,
    //     // ),
    //     // Mesh::from_gltf(
    //     //     "assets/models/cube_tex/BoxTextured.gltf",
    //     //     Vector(0., -2., 0.0),
    //     //     1.,
    //     // ),
    //     // Mesh::from_gltf(
    //     //     "assets/models/multi_uv/MultiUVTest.gltf",
    //     //     Vector(0., 2., 0.0),
    //     //     1.,
    //     // ),
    // ]
}

pub fn scene_light() -> Vec<Mesh> {
    vec![
        // primitives::create_grid("debug_grid", Vector(0., 0.0, 0.0), 20),
        // primitives::create_plane("plane_1", "", Vector(0., 0., 0.), 5.0),
        // Mesh::from_gltf(
        //     "assets/models/cube/Cube.gltf",
        //     Vector(0., 1.0, 0.0),
        //     1.,
        // ),
    ]
}
