mod colliders;
mod debug_scenes;
mod editor;
mod entities;
mod game_loop;
mod global;
mod input;
mod platform;
mod renderer;
mod time;
mod math;

use editor::Editor;
use entities::{Entities, Entity};
use game_loop::GameLoop;
use global::*;
use input::{Input, Key};
use renderer::{Renderer, Rgba};

fn main() {
    // Panic if platform not supported otherwise
    // log the current system and arch.
    platform::check_platform_supported();
    // Right now, we're using only glutin/winit for all desktop operating system.
    let mut platform = unsafe {
        platform::WinitPlatform::new(
            GAME_NAME,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            WITH_VSYNC,
            MULTISAMPLING,
        )
    };

    let mut game_loop = GameLoop::new();
    let mut input = Input::new();
    let mut entities = Entities::default();

    let mut renderer = Renderer::new(Rgba::new(0.1, 0.1, 0.2, 1.0));

    let mut editor = Editor::new();
    *VIEW_MATRIX.lock().unwrap() = editor.camera.get_look_at();

    // Add the debug scene for now.
    for mesh in debug_scenes::scene_mesh(&mut entities).into_iter() {
        entities.insert(mesh);
    }

    game_loop.start(|time| {
        platform.map_winit_inputs(&mut input);

        // Editor stuff here. With menu etc...
        editor.run(&mut entities, &platform, &mut input, &mut renderer, time);

        renderer.clear_screen();
        renderer.draw(&mut entities);

        // Actually "draw": swap the back buffer into the front buffer.
        platform.swap_buffers();
        platform.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
