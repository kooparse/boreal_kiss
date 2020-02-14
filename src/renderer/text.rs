use super::{Colors, Rgb, Vector};
use std::hash::{Hash, Hasher};

// Text that should be rendered with a specific
// font and a specifig position.
#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub position: Vector,
    pub color: Rgb,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            ..Default::default()
        }
    }

    #[allow(unused)]
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: Rgb) -> Self {
        self.color = color;
        self
    }

    #[allow(unused)]
    pub fn position(mut self, position: Vector) -> Self {
        self.position = position;
        self
    }

    #[allow(unused)]
    pub fn set_pos(&mut self, pos: Vector) {
        self.position = pos;
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_owned();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl Eq for Text {}
impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.font_size == other.font_size && self.content == other.content
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content.hash(state);
        (self.font_size as i32).hash(state);
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content: "".to_owned(),
            font_size: 26.,
            position: Vector(0., 0., 0.),
            color: Rgb::black(),
        }
    }
}
