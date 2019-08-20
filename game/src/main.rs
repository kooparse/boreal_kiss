mod constants;

use crate::constants::{
    DEFAULT_HEIGHT, DEFAULT_MULTISAMPLING, DEFAULT_WIDTH, GAME_NAME, WITH_VSYNC,
};
// #[cfg(target_os = "xbox_one")]
// use engine::window_xbox as window_x64;
#[cfg(any(target_os = "macos", target_os = "windows",))]
use engine::platform_x64_winit as platform_x64;
use engine::{
    game_loop,
    platform::{self, Platform},
    renderer::Renderer,
};

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

    let mut platform = Platform::from(platform_wrapper);
    let _renderer = Renderer::from(&platform);

    let mut game_loop = game_loop::GameLoop::new();

    // Get mutable ref of the inner platform,
    // we got an "PlatformWrapper" trait object.
    let window = platform.get_mut();

    game_loop.start(|_time, _fps| {

        window.poll_events();
        window.swap_buffers();

        window.should_close()
    });

    dbg!("Game exited correctly");
}
