use renderer::{Renderer, RendererOptions};

pub type DpiFactor = f64;

pub struct GameResolution {
    pub width: f64,
    pub height: f64,
}

// Platform store an object trait (window mostly).
// and different context associated with it.
pub struct Platform {
    pub inner_value: Box<dyn PlatformWrapper>,
}

/// Generic layer for custom platforms.
pub trait PlatformWrapper {
    fn get_dimension(&self) -> GameResolution;
    fn get_dpi_factor(&self) -> DpiFactor;
    fn should_close(&self) -> bool;
    fn swap_buffers(&self);
    fn poll_events(&mut self);
    fn load_opengl(&self) -> RendererOptions;
}

impl Platform {
    pub fn new(inner_value: Box<dyn PlatformWrapper>) -> Self {
        Self { inner_value }
    }
    /// Get ref of the inner platform.
    pub fn get(&self) -> &dyn PlatformWrapper {
        &*self.inner_value
    }
    /// Get mutable ref of the inner platform.
    pub fn get_mut(&mut self) -> &mut dyn PlatformWrapper {
        &mut *self.inner_value
    }
}

impl<'p, 't> From<&'p Platform> for Renderer {
    fn from(platform: &'p Platform) -> Self {
        let options = platform.get().load_opengl();
        Self::new(options)
    }
}

pub fn check_platform_supported() {
    let target_os: &str = if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "windows") {
        "Windows"
    } else {
        panic!("Target system not currently supported");
    };

    let target_arch: &str = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        panic!("Architecture not currently supported")
    };

    dbg!(target_os, target_arch);
}

pub fn is_desktop() -> bool {
    cfg!(target_os = "macos") || cfg!(target_os = "windows")
}
