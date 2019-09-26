use image;

pub type TextureDim = (u32, u32);

#[derive(Debug, Clone)]
pub struct Texture {
    pub raw: Vec<u8>,
    pub dim: TextureDim,
}

impl Texture {
    pub fn new(dim: (u32, u32), raw: Vec<u8>) -> Self {
        Self { raw, dim }
    }
    pub fn from_file(file_path: &str) -> Self {
        let img =
            image::open(file_path).expect("Failed to load texture in memory");

        Self {
            // Flip texture vertically so opengl uv mapping are set corretly.
            raw: img.flipv().raw_pixels(),
            dim: img.to_rgb().dimensions(),
        }
    }
}
