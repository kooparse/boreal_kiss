mod constants;

use crate::constants::{
    DEFAULT_HEIGHT, DEFAULT_MULTISAMPLING, DEFAULT_WIDTH, GAME_NAME, WITH_VSYNC,
};
// #[cfg(target_os = "xbox_one")]
// use engine::window_xbox as window_x64;
#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux"
))]
use engine::window_x64_winit as window_x64;
use engine::{game_loop, platform, window::Window};

fn main() {
    // Panic if platform not supported otherwise
    // log the current system and arch.
    platform::check_platform_supported();
    // Right now, we're using only winit for all desktop operating system.
    let inner_window = if platform::is_desktop() {
        window_x64::WinitWindow::new(
            GAME_NAME,
            (DEFAULT_WIDTH, DEFAULT_HEIGHT),
            WITH_VSYNC,
            DEFAULT_MULTISAMPLING,
        )
    } else {
        panic!("Only desktop platforms is currently supported");
    };

    let mut platform = platform::Platform::new(Window::from(inner_window));
    let mut game_loop = game_loop::GameLoop::new();
    let window = platform.window.get_mut();

    game_loop.start(|_time, fps| {
        dbg!(fps);

        window.poll_events();
        window.swap_buffers();

        window.should_close()
    });

    dbg!("Game exited correctly");
}
