pub type DpiFactor = f64;

pub struct WindowDimension {
    pub width: f64,
    pub height: f64,
}

/// Generic layer for custom windows.
pub trait WindowWrapper {
    fn get_dimension(&self) -> WindowDimension;
    fn get_dpi_factor(&self) -> DpiFactor;
    fn should_close(&self) -> bool;
    fn swap_buffers(&self);
    fn poll_events(&mut self);
}

/// Window object used for games.
/// Stored inner window as trait object (winit for macOS/Windows/Linux).
/// Probably customed ones later.
pub struct Window {
    pub inner_value: Box<WindowWrapper>,
}

impl Window {
    pub fn get(&self) -> &WindowWrapper {
        &*self.inner_value
    }

    pub fn get_mut(&mut self) -> &mut WindowWrapper {
        &mut *self.inner_value
    }
}
