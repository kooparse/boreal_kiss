use crate::window::Window;

// Platform store the window
// and different context associated with it.
pub struct Platform {
    pub window: Window,
}

impl Platform {
    pub fn new(window: Window) -> Self {
        Self { window }
    }
}

pub fn check_platform_supported() {
    let target_os: &str = if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        panic!("Target system not currently supported");
    };

    let target_arch: &str = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        panic!("Architecture not currently supported")
    };

    dbg!(target_os);
    dbg!(target_arch);
}

pub fn is_desktop() -> bool {
    cfg!(target_os = "macos")
        || cfg!(target_os = "windows")
        || cfg!(target_os = "linux")
}
