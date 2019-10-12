use std::hash::{Hash, Hasher};
use crate::{
    Vector, Rgb,
};

// Text that should be rendered with a specific
// font and a specifig position.
#[derive(Debug, Default)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub position: Vector,
    pub color: Rgb,
}

impl Eq for Text {}
impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.font_size == other.font_size 
            && self.content == other.content
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content.hash(state);
        (self.font_size as i32).hash(state);
    }
}

