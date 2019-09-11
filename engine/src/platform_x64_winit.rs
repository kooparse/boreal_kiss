/// Platform with 64 bits instruction set, using Winit for window creation.
/// Windows, macOS and Linux are supported thanks to winit crate.
///
/// We are using this crate for now, even if we don't have a total control
/// over the creation of window on those targets.
use crate::platform::{DpiFactor, GameResolution, Platform, PlatformWrapper};
use gl;
use glutin::{
    dpi, Api, ContextBuilder, ContextWrapper, Event, EventsLoop, GlRequest,
    PossiblyCurrent, VirtualKeyCode, Window as GlutinWindow, WindowBuilder,
    WindowEvent,
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
            context,
            event_loop,
        }
    }
}

impl PlatformWrapper for WinitPlatform {
    fn get_dimension(&self) -> GameResolution {
        let window = self.context.window();
        let inner_size = window.get_inner_size().unwrap();

        GameResolution {
            width: inner_size.width,
            height: inner_size.height,
        }
    }

    fn get_dpi_factor(&self) -> DpiFactor {
        self.context.window().get_hidpi_factor()
    }

    fn swap_buffers(&self) {
        self.context
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn load_opengl(&self) -> RendererOptions {
        let dim = self.get_dimension();
        let pixel_format = self.context.get_pixel_format();

        gl::load_with(|symbol| {
            self.context.get_proc_address(symbol) as *const _
        });

        RendererOptions::new(
            pixel_format.multisampling.is_some(),
            true,
            Color(0.1, 0.1, 0.2, 1.0),
            (dim.width, dim.height)
        )
    }

    fn poll_events(&mut self) {
        let mut should_close = false;

        self.event_loop
            .poll_events(|glutin_event| match &glutin_event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        should_close = true;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            should_close = keycode == VirtualKeyCode::Escape;
                        }
                    }
                    _ => (),
                },
                _ => (),
            });

        self.should_close = should_close;
    }
}

impl From<WinitPlatform> for Platform {
    fn from(window: WinitPlatform) -> Self {
        Self::new(Box::new(window))
    }
}
