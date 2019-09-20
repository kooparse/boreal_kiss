/// Platform with 64 bits instruction set, using Winit for window creation.
/// Windows, macOS and Linux are supported thanks to winit crate.
///
/// We are using this crate for now, even if we don't have a total control
/// over the creation of window on those targets.
use crate::input::{Cursor, Input, Key, Modifier, MouseButton};
use crate::platform::{GameResolution, Platform, PlatformWrapper};
use gl;
use glutin::{
    dpi, Api, ContextBuilder, ContextWrapper, DeviceEvent, ElementState, Event,
    EventsLoop, GlRequest, MouseButton as GlMouseButton, PossiblyCurrent,
    VirtualKeyCode, Window as GlutinWindow, WindowBuilder, WindowEvent,
};
use renderer::{Color, RendererOptions};
use std::convert::From;

/// Construct a window for all desktop with the
/// opengl v4.1 loaded in the context. The 4.1 version is
/// the latest opengl version available for the currently latest
/// macOS version.
pub struct WinitPlatform {
    should_close: bool,
    event_loop: EventsLoop,
    window_size_changed: bool,
    context: ContextWrapper<PossiblyCurrent, GlutinWindow>,
}

impl WinitPlatform {
    pub fn new(
        title: &str,
        (width, height): (u32, u32),
        with_vsync: bool,
        multisampling: u16,
    ) -> Self {
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

        Self {
            should_close: false,
            window_size_changed: false,
            context,
            event_loop,
        }
    }
}

impl PlatformWrapper for WinitPlatform {
    fn get_dimension(&self) -> GameResolution {
        let window = self.context.window();
        let dpi = window.get_hidpi_factor();
        let inner_size = window.get_inner_size().unwrap();

        GameResolution {
            width: inner_size.width,
            height: inner_size.height,
            dpi,
        }
    }

    fn swap_buffers(&self) {
        self.context
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }

    fn on_resize(&self, callback: &mut dyn FnMut(GameResolution)) {
        if self.window_size_changed {
            callback(self.get_dimension())
        }
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    /// Hide and Grab the cursor.
    fn hide_cursor(&self, is_hide: bool) {
        self.context
            .window()
            .grab_cursor(is_hide)
            .expect("Error when grabbing the cursor");

        self.context.window().hide_cursor(is_hide);
    }

    fn load_opengl(&self) -> RendererOptions {
        let pixel_format = self.context.get_pixel_format();

        gl::load_with(|symbol| {
            self.context.get_proc_address(symbol) as *const _
        });

        RendererOptions::new(
            pixel_format.multisampling.is_some(),
            true,
            Color(0.1, 0.1, 0.2, 1.0),
        )
    }

    fn update_inputs(&mut self, game_input: &mut Input) {
        let mut window_size_changed = false;
        let mut should_close = false;
        game_input.cursor.has_moved = false;

        self.event_loop
            .poll_events(|glutin_event| match &glutin_event {
                Event::DeviceEvent { event, .. } => match &event {
                    DeviceEvent::MouseMotion { delta, .. } => {
                        let moved = Cursor {
                            delta: *delta,
                            has_moved: true,
                            ..game_input.cursor
                        };

                        game_input.update_cursor_position(moved);
                    }
                    _ => (),
                },
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

        self.window_size_changed = window_size_changed;
        self.should_close = should_close;
    }
}

impl From<WinitPlatform> for Platform {
    fn from(window: WinitPlatform) -> Self {
        Self::new(Box::new(window))
    }
}
