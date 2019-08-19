mod opengl;
use crate::platform::Platform;

pub type Color = (f32, f32, f32, f32);

pub struct RendererOptions {
    with_multisampling: bool,
    default_color: Color,
}

pub struct Renderer<'p> {
    platform: &'p Platform,
    options: RendererOptions,
}

impl RendererOptions {
    pub fn new(with_multisampling: bool, default_color: Color) -> Self {
        Self {
            with_multisampling,
            default_color,
        }
    }
}

impl<'p> Renderer<'p> {
    pub fn new(platform: &'p Platform, options: RendererOptions) -> Self {
        // Panic if opengl functions not loaded.
        opengl::get_opengl_loaded();

        opengl::set_multisampling(options.with_multisampling);
        opengl::clear_color(options.default_color);

        Self { options, platform }
    }
}
