use image;

pub type TextureDim = (u32, u32);

pub struct Texture<'p> {
    #[allow(dead_code)]
    file_path: &'p str,
    pub raw: Vec<u8>,
    pub dim: TextureDim,
}

impl<'p> Texture<'p> {
    pub fn new(file_path: &'p str) -> Self {
        let img =
            image::open(file_path).expect("Failed to load texture in memory");

        Self {
            file_path,
            // Flip texture vertically so opengl uv mapping are set corretly.
            raw: img.flipv().raw_pixels(),
            dim: img.to_rgb().dimensions(),
        }
    }
}
