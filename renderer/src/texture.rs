use super::opengl;
use image;

pub type TextureId = u32;

pub struct TextureDim {
    pub width: u32,
    pub height: u32,
}

pub struct UV {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Texture<'p> {
    #[allow(dead_code)]
    file_path: &'p str,
    pub uv: Vec<UV>,
    pub id: TextureId,
    pub raw: Vec<u8>,
    pub dim: TextureDim,
}

impl<'p> Texture<'p> {
    pub fn new(file_path: &'p str, uv: Vec<UV>) -> Self {
        let img =
            image::open(file_path).expect("Failed to load texture in memory");

        let id = opengl::generate_texture();
        let dim = img.to_rgb().dimensions();
        let dim = TextureDim {
            width: dim.0,
            height: dim.1,
        };
        let raw = img.raw_pixels();

        Self {
            file_path,
            uv,
            id,
            raw,
            dim,
        }
    }
}
