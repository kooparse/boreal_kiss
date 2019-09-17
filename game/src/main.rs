mod constants;

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
    let mut renderer = Renderer::from(&platform);
    let mut input = Input::new();

    let _ids = renderer.add_meshes(vec![
        primitives::create_triangle_object(
            "plane_1",
            "game/assets/textures/pos_debug.png",
            glm::vec3(1., 0.0, 0.0),
            1.0,
        ),
        primitives::create_triangle_object(
            "plane_2",
            "game/assets/textures/grid_debug.png",
            glm::vec3(-1., 0., 0.),
            0.4,
        ),
    ]);

    // Get mutable ref of the inner platform,
    // we got an "PlatformWrapper" trait object.
    let window = platform.get_mut();

    game_loop.start(|_time, _fps| {
        window.update_inputs(&mut input);

        renderer.clear_screen();
        renderer.draw();

        window.on_resize(&mut |resolusion| {
            renderer.platform_resized(resolusion.width, resolusion.height);
        });

        input.pressed_once(Key::A, || renderer.toggle_mesh(_ids[0]));
        input.clicked(MouseButton::Left, |c_pos| {
            dbg!(c_pos);
        });

        window.swap_buffers();
        window.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
