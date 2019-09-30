mod constants;
mod state;

use crate::constants::{
    DEFAULT_HEIGHT, DEFAULT_MULTISAMPLING, DEFAULT_WIDTH, GAME_NAME, WITH_VSYNC,
};
// #[cfg(target_os = "xbox_one")]
// use engine::window_xbox as window_x64;
use editor::Editor;
use engine::{
    game_loop::GameLoop,
    input::{Input, Key},
    platform::{self, Platform},
    platform_x64_winit as platform_x64,
};
use nalgebra_glm as glm;
use renderer::Renderer;
use state::GameState;

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

    let mut editor = Editor::default();
    editor.init(&mut renderer);

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

        editor.update_events(&mut input, &mut renderer, &state.render_state);
        state.render_state.view = editor.move_camera(&mut input, window, time);

        renderer.clear_screen();
        renderer.draw(&state.render_state);

        window.swap_buffers();
        window.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
