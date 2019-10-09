use super::{Pos2D, Rgb};

// Text that should be rendered with a specific
// font and a specifig position.
#[derive(Default)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub position: Pos2D,
    pub color: Rgb,
}
