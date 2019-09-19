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
    let state = GameState::default();

    let mut renderer =
        Renderer::new(platform.get().load_opengl(), &state.render_state);

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
        primitives::create_line("line_1", glm::vec3(0., 0., 0.)),
    ]);

    // Get mutable ref of the inner platform,
    // we got an "PlatformWrapper" trait object.
    let window = platform.get_mut();

    game_loop.start(|_time, _fps| {
        window.update_inputs(&mut input);

        renderer.clear_screen();
        renderer.draw();

        window.on_resize(&mut |res| {
            let mut render_state = state.render_state.borrow_mut();

            renderer.update_viewport(res.width, res.height, res.dpi);
            render_state.projection = glm::perspective(
                (res.width / res.height) as f32,
                45.0,
                0.1,
                100.0,
            );
        });

        input.pressed_once(Key::W, || {
            let mut render_state = state.render_state.borrow_mut();
            render_state.view =
                glm::translate(&render_state.view, &glm::vec3(0., 0.2, 0.));
        });

        input.pressed_once(Key::S, || {
            let mut render_state = state.render_state.borrow_mut();
            render_state.view =
                glm::translate(&render_state.view, &glm::vec3(0., -0.2, 0.));
        });

        input.pressed_once(Key::D, || {
            let mut render_state = state.render_state.borrow_mut();
            render_state.view =
                glm::translate(&render_state.view, &glm::vec3(-0.2, 0., 0.));
        });

        input.pressed_once(Key::A, || {
            let mut render_state = state.render_state.borrow_mut();
            render_state.view =
                glm::translate(&render_state.view, &glm::vec3(0.2, 0., 0.));
        });

        input.pressed_once(Key::X, || {
            let mut render_state = state.render_state.borrow_mut();
            render_state.view =
                glm::translate(&render_state.view, &glm::vec3(0., 0., -0.2));
        });

        input.clicked(MouseButton::Left, |c_pos| {
            dbg!(c_pos);
        });

        window.swap_buffers();
        window.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
