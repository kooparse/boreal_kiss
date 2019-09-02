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

pub struct TexCoords(pub Vec<UV>);

pub struct Texture<'p> {
    #[allow(dead_code)]
    file_path: &'p str,
    pub uv: TexCoords,
    pub id: Option<TextureId>,
    pub raw: Option<Vec<u8>>,
    pub dim: Option<TextureDim>,
}

impl<'p> Texture<'p> {
    pub fn new(file_path: &'p str, uv: TexCoords) -> Self {
        let img =
            image::open(file_path).expect("Failed to load texture in memory");

        let id = Some(opengl::generate_texture());
        let dim = img.to_rgb().dimensions();
        let dim = Some(TextureDim {
            width: dim.0,
            height: dim.1,
        });
        let raw = Some(img.raw_pixels());

        Self {
            file_path,
            uv,
            id,
            raw,
            dim,
        }
    }

    /// Unload texture from the system memory. Used often after
    /// passing the texture bytes into the gpu.
    pub fn mem_free(&mut self) {
        self.raw = None;
        self.dim = None;
        // TODO: free texture id on opengl too.
        self.id = None;
    }
}
