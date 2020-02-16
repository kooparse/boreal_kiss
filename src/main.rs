mod camera;
mod colliders;
mod debug_scenes;
mod editor;
mod entities;
mod game_loop;
mod global;
mod gui;
mod input;
mod math;
mod platform;
mod player;
mod renderer;
mod tilemap;
mod time;
mod wall;

use camera::Camera;
use editor::Editor;
use entities::{Entities, Entity};
use game_loop::GameLoop;
use global::*;
use gui::{Button, Container, TextInput, GUI};
use input::{Input, Key};
use renderer::{Colors, Font, Renderer, Rgb, Rgba, Text};
use tilemap::init_world_and_player;

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

    let mut font = Font::new(
        "assets/fonts/Helvetica/helvetica.json",
        "assets/fonts/Helvetica/helvetica.png",
    );

    let mut counter = 0;

    let submit = Button::new(Text::new("counter: 0").color(Rgb::black()))
        .width(120.)
        .height(40.)
        .margin(2.5)
        .bg_color(Rgba::new(1., 0.5, 0.5, 1.))
        .callback(move |text| {
            counter += 1;
            text.set_content(&format!("counter: {}", counter));
        });

    let save = Button::new(Text::new("save"))
        .width(120.)
        .height(40.)
        .margin(2.5)
        .bg_color(Rgba::new(1., 1., 0.5, 1.))
        .text_color(Rgb::new(0., 0., 0.))
        .callback(move |_| {
            dbg!("clicked!");
        });

    let test = Button::new(Text::new("test"))
        .width(120.)
        .height(40.)
        .margin(2.5)
        .bg_color(Rgba::new(0.5, 1., 0.5, 1.))
        .text_color(Rgb::new(0., 0., 1.))
        .callback(move |_| {
            dbg!("clicked!");
        });

    let text_input = TextInput::new()
        .label(Text::new("Text label :").color(Rgb::white()))
        .value(Text::new("2").color(Rgb::white()))
        .padding(8.)
        .only_numbers(true)
        .on_update(|t| {
            dbg!(t.content());
        });

    let col_left = Container::row().margin(5.).push(test);
    let row_right = Container::col()
        .margin(5.)
        .push(text_input)
        .push(save)
        .push(submit);

    let container = Container::row()
        .width(1200.)
        .height(800.)
        .margin(5.)
        .push(col_left)
        .push(row_right);

    let mut gui = GUI::new().add_elem(container);
    gui.draw(&mut font);

    let (mut world, mut player) = init_world_and_player(&mut entities);
    let mut camera = Camera::new(&player);

    let mut renderer =
        Renderer::new(Rgba::new(0.53, 0.81, 0.92, 1.0), &mut entities);

    let mut editor = Editor::new();
    *VIEW_MATRIX.lock().unwrap() = editor.camera.get_look_at();

    let mut is_debug_mode = false;

    game_loop.start(|time| {
        platform.map_winit_inputs(&mut input);

        if input.modifiers.ctrl && input.is_pressed_once(Key::L) {
            is_debug_mode = !is_debug_mode;
        }

        gui.on_event(&mut input);

        // Editor stuff here. With menu etc...
        player.update_player(
            &time,
            &camera,
            &mut input,
            &mut world,
            &mut entities,
        );

        if !is_debug_mode {
            *VIEW_MATRIX.lock().unwrap() =
                camera.follow_player(&player, &mut input, &time);
        } else {
            *VIEW_MATRIX.lock().unwrap() = editor.camera.get_look_at();
            editor.run(
                &mut entities,
                &platform,
                &mut input,
                &mut renderer,
                time,
            );
        }

        renderer.clear_screen();
        renderer.draw(&mut entities, &world, &player);
        gui.draw(&mut font);

        // Actually "draw": swap the back buffer into the front buffer.
        platform.swap_buffers();
        platform.should_close() || input.is_pressed(Key::Esc)
    });

    dbg!("Game exited correctly");
}
