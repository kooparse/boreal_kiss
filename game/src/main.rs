mod constants;
mod state;

use crate::constants::{
    DEFAULT_HEIGHT, DEFAULT_MULTISAMPLING, DEFAULT_WIDTH, GAME_NAME, WITH_VSYNC,
};
// #[cfg(target_os = "xbox_one")]
// use engine::window_xbox as window_x64;
use engine::{
    game_loop::GameLoop,
    input::{Input, Key, MouseButton},
    platform::{self, Platform},
    platform_x64_winit as platform_x64,
};
use nalgebra_glm as glm;
use renderer::{primitives, Renderer};
use state::GameState;
#[cfg(any(target_os = "macos", target_os = "windows",))]

fn main() {
    // Panic if platform not supported otherwise
    // log the current system and arch.
    platform::check_platform_supported();
    // Right now, we're using only glutin/winit for all desktop operating system.
    let platform_wrapper = if platform::is_desktop() {
        platform_x64::WinitPlatform::new(
            GAME_NAME,
            (DEFAULT_WIDTH, DEFAULT_HEIGHT),
            WITH_VSYNC,
            DEFAULT_MULTISAMPLING,
        )
    } else {
        panic!("Only desktop platforms is currently supported");
    };

    let mut game_loop = GameLoop::new();
    let mut platform = Platform::from(platform_wrapper);
    let mut input = Input::new();
    let mut state = GameState::default();
    let mut renderer = Renderer::new(platform.get().load_opengl());

    // Get mutable ref of the inner platform,
    // we got an "PlatformWrapper" trait object.
    let window = platform.get_mut();

    game_loop.start(|time| {
        window.update_inputs(&mut input);

        window.on_resize(&mut |res| {
            renderer.update_viewport(&res);
            state.render_state.projection = glm::perspective(
                (res.width / res.height) as f32,
                45.0,
                0.1,
                100.0,
            );
        });

        state.move_camera(&mut input, window, time);

        input.clicked(MouseButton::Left, |cursor| {
            let (origin, direction) = state.cast_ray(cursor);
            renderer.add_ray(origin, direction, 100f32);
        });

        input.pressed_once(Key::Key1, || {
            renderer.flush();
            renderer.add_meshes(vec![
                primitives::create_plane(
                    "plane_1",
                    "game/assets/textures/pos_debug.png",
                    glm::vec3(0., 0.0, 0.0),
                    1.0,
                ),
                primitives::load_mesh(
                    "game/assets/models/cube/Cube.gltf",
                    glm::vec3(2., 0.0, 0.0),
                    0.7,
                ),
                primitives::load_mesh(
                    "game/assets/models/cube_color/BoxVertexColors.gltf",
                    glm::vec3(-2., 0.0, 0.0),
                    0.7,
                ),
                primitives::load_mesh(
                    "game/assets/models/cube_tex/BoxTextured.gltf",
                    glm::vec3(0., -2., 0.0),
                    1.,
                ),
                primitives::load_mesh(
                    "game/assets/models/multi_uv/MultiUVTest.gltf",
                    glm::vec3(0., 2., 0.0),
                    1.,
                ),
            ]);
        });

        input.pressed_once(Key::Key2, || {
            renderer.flush();
            renderer.add_meshes(vec![
                primitives::create_plane(
                    "plane_1",
                    "game/assets/textures/pos_debug.png",
                    glm::vec3(0., 1.2, 0.0),
                    1.0,
                ),
                primitives::load_mesh(
                    "game/assets/models/cube/Cube.gltf",
                    glm::vec3(0., 0.0, 0.0),
                    0.7,
                ),
                primitives::load_mesh(
                    "game/assets/models/cube_color/BoxVertexColors.gltf",
                    glm::vec3(2., 0.0, 0.0),
                    0.7,
                ),
                primitives::load_mesh(
                    "game/assets/models/cube_tex/BoxTextured.gltf",
                    glm::vec3(0., 2., 0.0),
                    1.,
                ),
                primitives::load_mesh(
                    "game/assets/models/multi_uv/MultiUVTest.gltf",
                    glm::vec3(0., -2., 0.0),
                    1.,
                ),
            ]);
        });

        renderer.clear_screen();
        renderer.draw(&state.render_state);

        window.swap_buffers();
        window.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
