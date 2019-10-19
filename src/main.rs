mod debug_scenes;
mod editor;
mod game_loop;
mod global;
mod input;
mod memory_arena;
mod platform;
mod renderer;
mod time;

use editor::Editor;
use game_loop::GameLoop;
use global::*;
use input::{Input, Key};
use renderer::{Renderer, Rgba};

fn main() {
    // Panic if platform not supported otherwise
    // log the current system and arch.
    platform::check_platform_supported();
    // Right now, we're using only glutin/winit for all desktop operating system.
    let mut platform = if platform::is_desktop() {
        unsafe {
            platform::WinitPlatform::new(
                GAME_NAME,
                (SCREEN_WIDTH, SCREEN_HEIGHT),
                WITH_VSYNC,
                MULTISAMPLING,
            )
        }
    } else {
        panic!("Only desktop platforms is currently supported");
    };

    let mut game_loop = GameLoop::new();
    let mut input = Input::new();
    let mut renderer = Renderer::new(Rgba::new(0.1, 0.1, 0.2, 1.0));

    let mut editor = Editor::new(&mut renderer);
    *VIEW_MATRIX.lock().unwrap() = editor.camera.get_look_at();

    let debug_mesh_scene = debug_scenes::scene_mesh();
    renderer.add_meshes(debug_mesh_scene);

    game_loop.start(|time| {
        platform.map_winit_inputs(&mut input);

        // Editor stuff here. With menu etc...
        editor.run(&platform, &mut input, &mut renderer, time);

        renderer.clear_screen();
        renderer.draw();

        // Actually "draw": swap the back buffer into the front buffer.
        platform.swap_buffers();
        platform.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
