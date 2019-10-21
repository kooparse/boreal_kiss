/// Platform with 64 bits instruction set, using Winit for window creation.
/// Windows, macOS and Linux are supported thanks to winit crate.
///
/// We are using this crate for now, even if we don't have a total control
/// over the creation of window on those targets.
use super::input::{Cursor, Input, Key, Modifier, MouseButton};
// use super::platform::{Platform, PlatformWrapper};
use crate::global::*;
use gl;
use glutin::{
    dpi, Api, ContextBuilder, ContextWrapper, DeviceEvent, ElementState, Event,
    EventsLoop, GlRequest, MouseButton as GlMouseButton, PossiblyCurrent,
    VirtualKeyCode, Window as GlutinWindow, WindowBuilder, WindowEvent,
};
use nalgebra_glm as glm;
use std::convert::From;

/// Construct a window for all desktop with the
/// opengl v4.1 loaded in the context. The 4.1 version is
/// the latest opengl version available for the currently latest
/// macOS version.
pub struct WinitPlatform {
    should_close: bool,
    event_loop: EventsLoop,
    context: ContextWrapper<PossiblyCurrent, GlutinWindow>,
}

impl WinitPlatform {
    pub fn new(
        title: &str,
        (width, height): (f32, f32),
        with_vsync: bool,
        multisampling: u16,
    ) -> Self {
        if !is_desktop() {
            panic!("Only desktop platforms is currently supported");
        }

        // Dimensions based on factor dpi (LogicalSize).
        let dimensions =
            dpi::LogicalSize::new(f64::from(width), f64::from(height));

        let builder = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dimensions);

        let event_loop = EventsLoop::new();

        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)))
            .with_vsync(with_vsync)
            .with_multisampling(multisampling)
            .build_windowed(builder, &event_loop)
            .unwrap();

        let context = unsafe { context.make_current().unwrap() };

        // Load gl function pointers.
        gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

        Self {
            should_close: false,
            context,
            event_loop,
        }
    }

    pub fn swap_buffers(&self) {
        self.context
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }

    pub fn on_resize(&self) {
        let window = self.context.window();
        let dpi = window.get_hidpi_factor();
        let inner_size = window.get_inner_size().unwrap();

        unsafe {
            SCREEN_WIDTH = inner_size.width as f32;
            SCREEN_HEIGHT = inner_size.height as f32;
            SCREEN_DPI = dpi as u32;

            *PERSPECTIVE_MATRIX.lock().unwrap() = glm::perspective(
                SCREEN_WIDTH / SCREEN_HEIGHT,
                45.0,
                0.1,
                100.0,
            );
            *ORTHO_MATRIX.lock().unwrap() =
                glm::ortho(0., SCREEN_WIDTH, 0., SCREEN_HEIGHT, -1., 1.);

            // opengl::set_viewport(
            //     (width * dpi) as i32,
            //     (height * dpi) as i32,
            // );
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    /// Hide and Grab the cursor.
    pub fn hide_cursor(&self, is_hide: bool) {
        self.context
            .window()
            .grab_cursor(is_hide)
            .expect("Error when grabbing the cursor");

        self.context.window().hide_cursor(is_hide);
    }

    // Map winit input to our own input layer.
    pub fn map_winit_inputs(&mut self, game_input: &mut Input) {
        let mut window_size_changed = false;
        let mut should_close = false;
        game_input.cursor.has_moved = false;

        self.event_loop
            .poll_events(|glutin_event| match &glutin_event {
                Event::DeviceEvent { event, .. } => {
                    if let DeviceEvent::MouseMotion { delta, .. } = &event {
                        let moved = Cursor {
                            delta: *delta,
                            has_moved: true,
                            ..game_input.cursor
                        };

                        game_input.update_cursor_position(moved);
                    }
                }
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Resized(_) => {
                            window_size_changed = true;
                        }
                        WindowEvent::CloseRequested => {
                            should_close = true;
                        }
                        // LogicalSized => scaled by dpi factor.
                        WindowEvent::CursorMoved { position, .. } => {
                            let moved = Cursor {
                                position: (position.x, position.y),
                                has_moved: true,
                                ..game_input.cursor
                            };

                            game_input.update_cursor_position(moved);
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let is_clicked = *state == ElementState::Pressed;

                            match button {
                                GlMouseButton::Right => game_input
                                    .update_mouse(
                                        MouseButton::Right,
                                        is_clicked,
                                    ),
                                GlMouseButton::Left => game_input.update_mouse(
                                    MouseButton::Left,
                                    is_clicked,
                                ),
                                GlMouseButton::Middle => game_input
                                    .update_mouse(
                                        MouseButton::Middle,
                                        is_clicked,
                                    ),
                                _ => (),
                            }
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            let is_pressed =
                                input.state == ElementState::Pressed;

                            if let Some(keycode) = input.virtual_keycode {
                                match keycode {
                                    VirtualKeyCode::A => game_input
                                        .update_key(Key::A, is_pressed),
                                    VirtualKeyCode::B => game_input
                                        .update_key(Key::B, is_pressed),
                                    VirtualKeyCode::C => game_input
                                        .update_key(Key::C, is_pressed),
                                    VirtualKeyCode::D => game_input
                                        .update_key(Key::D, is_pressed),
                                    VirtualKeyCode::E => game_input
                                        .update_key(Key::E, is_pressed),
                                    VirtualKeyCode::F => game_input
                                        .update_key(Key::F, is_pressed),
                                    VirtualKeyCode::G => game_input
                                        .update_key(Key::G, is_pressed),
                                    VirtualKeyCode::H => game_input
                                        .update_key(Key::H, is_pressed),
                                    VirtualKeyCode::I => game_input
                                        .update_key(Key::I, is_pressed),
                                    VirtualKeyCode::J => game_input
                                        .update_key(Key::J, is_pressed),
                                    VirtualKeyCode::K => game_input
                                        .update_key(Key::K, is_pressed),
                                    VirtualKeyCode::L => game_input
                                        .update_key(Key::L, is_pressed),
                                    VirtualKeyCode::M => game_input
                                        .update_key(Key::M, is_pressed),
                                    VirtualKeyCode::O => game_input
                                        .update_key(Key::O, is_pressed),
                                    VirtualKeyCode::P => game_input
                                        .update_key(Key::P, is_pressed),
                                    VirtualKeyCode::Q => game_input
                                        .update_key(Key::Q, is_pressed),
                                    VirtualKeyCode::R => game_input
                                        .update_key(Key::R, is_pressed),
                                    VirtualKeyCode::S => game_input
                                        .update_key(Key::S, is_pressed),
                                    VirtualKeyCode::T => game_input
                                        .update_key(Key::T, is_pressed),
                                    VirtualKeyCode::U => game_input
                                        .update_key(Key::U, is_pressed),
                                    VirtualKeyCode::V => game_input
                                        .update_key(Key::V, is_pressed),
                                    VirtualKeyCode::W => game_input
                                        .update_key(Key::W, is_pressed),
                                    VirtualKeyCode::X => game_input
                                        .update_key(Key::X, is_pressed),
                                    VirtualKeyCode::Y => game_input
                                        .update_key(Key::Y, is_pressed),
                                    VirtualKeyCode::Z => game_input
                                        .update_key(Key::Z, is_pressed),
                                    VirtualKeyCode::Key1 => game_input
                                        .update_key(Key::Key1, is_pressed),
                                    VirtualKeyCode::Key2 => game_input
                                        .update_key(Key::Key2, is_pressed),
                                    VirtualKeyCode::Key3 => game_input
                                        .update_key(Key::Key3, is_pressed),
                                    VirtualKeyCode::Key4 => game_input
                                        .update_key(Key::Key4, is_pressed),
                                    VirtualKeyCode::Key5 => game_input
                                        .update_key(Key::Key5, is_pressed),
                                    VirtualKeyCode::Key6 => game_input
                                        .update_key(Key::Key6, is_pressed),
                                    VirtualKeyCode::Key7 => game_input
                                        .update_key(Key::Key7, is_pressed),
                                    VirtualKeyCode::Key8 => game_input
                                        .update_key(Key::Key8, is_pressed),
                                    VirtualKeyCode::Key9 => game_input
                                        .update_key(Key::Key9, is_pressed),
                                    VirtualKeyCode::Key0 => game_input
                                        .update_key(Key::Key0, is_pressed),
                                    VirtualKeyCode::Escape => {
                                        game_input
                                            .update_key(Key::Esc, is_pressed);
                                    }
                                    VirtualKeyCode::Left => {
                                        game_input
                                            .update_key(Key::Left, is_pressed);
                                    }
                                    VirtualKeyCode::Down => {
                                        game_input
                                            .update_key(Key::Down, is_pressed);
                                    }
                                    VirtualKeyCode::Up => {
                                        game_input
                                            .update_key(Key::Up, is_pressed);
                                    }
                                    VirtualKeyCode::Right => {
                                        game_input
                                            .update_key(Key::Right, is_pressed);
                                    }
                                    VirtualKeyCode::Back => {
                                        game_input
                                            .update_key(Key::Bspc, is_pressed);
                                    }
                                    VirtualKeyCode::Return => {
                                        game_input
                                            .update_key(Key::Enter, is_pressed);
                                    }
                                    VirtualKeyCode::Tab => {
                                        game_input
                                            .update_key(Key::Tab, is_pressed);
                                    }
                                    VirtualKeyCode::Space => {
                                        game_input
                                            .update_key(Key::Space, is_pressed);
                                    }
                                    _ => (),
                                };

                                game_input.set_modifier(Modifier {
                                    ctrl: input.modifiers.ctrl,
                                    shift: input.modifiers.shift,
                                    alt: input.modifiers.alt,
                                    os: input.modifiers.logo,
                                });
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            });

        if window_size_changed {
            self.on_resize();
        }

        self.should_close = should_close;
    }
}

pub fn check_platform_supported() {
    let _target_os: &str = if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "windows") {
        "Windows"
    } else {
        panic!("Target system not currently supported");
    };

    let _target_arch: &str = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        panic!("Architecture not currently supported")
    };

    dbg!(_target_os, _target_arch);
}

pub fn is_desktop() -> bool {
    cfg!(target_os = "macos") || cfg!(target_os = "windows")
}
