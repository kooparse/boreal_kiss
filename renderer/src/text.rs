use super::{Pos2D, Rgb};

// Text that should be rendered with a specific
// font and a specifig position.
#[derive(Default)]
pub struct Text {
    pub content: String,
    pub position: Pos2D,
    pub font_attached: String,
    pub color: Rgb,
}
