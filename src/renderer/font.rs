use super::{
    opengl,
    shaders::{self, ShaderType},
    text::Text,
    texture::Texture,
    GpuBound,
};
use crate::global::*;
use nalgebra_glm as glm;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Debug, Deserialize)]
pub struct Character {
    #[serde(alias = "x")]
    atlas_pos_x: f32,
    #[serde(alias = "y")]
    atlas_pos_y: f32,
    width: f32,
    height: f32,
    #[serde(alias = "originY")]
    origin_y: f32,
    #[serde(alias = "originX")]
    origin_x: f32,
    advance: f32,
}

#[derive(Debug, Deserialize)]
pub struct Font {
    name: String,
    characters: HashMap<String, Character>,
    #[serde(skip_deserializing)]
    atlas_texture: Texture,
    #[serde(alias = "width")]
    atlas_width: f32,
    #[serde(alias = "height")]
    atlas_height: f32,
    #[serde(alias = "bold")]
    is_bold: bool,
    pub size: f32,
    #[serde(skip_deserializing)]
    text_caching: HashMap<String, GpuBound>,
}

impl Font {
    /// Take a metadata path and the texture atlas path.
    pub fn new(metadata: &str, atlas: &str) -> Self {
        let file =
            File::open(metadata).expect("Error while openning the font.");

        let reader = BufReader::new(file);

        let mut font: Font =
            serde_json::from_reader(reader).expect("Error while reading JSON");

        let texture = Texture::from_file(atlas);
        font.atlas_texture = texture;

        font
    }

    /// Compute the text length in pixels.
    pub fn text_length(&self, text: &Text) -> (f32, f32) {
        let scale = text.font_size / self.size;
        let mut cursor = 0.;
        let mut height = 0.;

        if text.is_empty() {
            return (0., height);
        }

        let content: Vec<(f32, f32)> = text
            .content
            .split("")
            .filter_map(|c| {
                self.characters.get(c).and_then(|l| {
                    let x_pos = (cursor - l.origin_x) * scale;
                    let y_pos = (0. - (l.height - l.origin_y)) * scale;
                    let h = y_pos + (l.height * scale);
                    let w = l.width * scale;

                    if height <= h {
                        height = h;
                    }

                    if height <= h {
                        height = h;
                    }

                    cursor += l.advance;
                    return Some((x_pos, x_pos + w));
                })
            })
            .collect();

        let width = content[content.len() - 1].1 - content[0].0;

        (width, height)
    }

    pub fn render(&mut self, text: &Text) {
        let scale = text.font_size / self.size;

        // Caching system.
        // If this text was previously rendered, we just use our existing
        // gpu data.
        if let Some(gpu_bound) = self.text_caching.get(&text.content) {
            self.to_opengl(&text, &gpu_bound);
            return;
        }

        let mut cursor = 0.;
        let mut vertices: Vec<f32> = vec![];

        text.content
            .split("")
            // If character not found in our atlas, we skip.
            .filter(|l| self.characters.get(l.to_owned()).is_some())
            .for_each(|letter| {
                // We can unwrap it safely now.
                let letter = self.characters.get(letter).unwrap();

                let (top_left, top_right, bottom_left, bottom_right) = {
                    let top_left = (
                        letter.atlas_pos_x / self.atlas_width,
                        letter.atlas_pos_y / self.atlas_height,
                    );

                    let top_right = (
                        top_left.0 + (letter.width / self.atlas_width),
                        top_left.1,
                    );

                    let bottom_left = (
                        top_left.0,
                        top_left.1 + (letter.height / self.atlas_height),
                    );

                    let bottom_right = (top_right.0, bottom_left.1);

                    (top_left, top_right, bottom_left, bottom_right)
                };

                let x_pos = (cursor - letter.origin_x) * scale;
                // 0 is our baseline.
                let y_pos = (0. - (letter.height - letter.origin_y)) * scale;
                let width = letter.width * scale;
                let height = letter.height * scale;

                // Quad data for our character.
                #[rustfmt::skip]
                let character_quad: [f32; 24] = [
                    x_pos, y_pos + height,  top_left.0, top_left.1,
                    x_pos,  y_pos,          bottom_left.0, bottom_left.1,
                    x_pos + width, y_pos,   bottom_right.0, bottom_right.1,

                    x_pos, y_pos + height,  top_left.0, top_left.1,
                    x_pos + width, y_pos,   bottom_right.0, bottom_right.1,
                    x_pos + width, y_pos + height, top_right.0, top_right.1,
                ];

                vertices.extend_from_slice(&character_quad);

                cursor += letter.advance;
            });

        let (vao, vbo, tex_id) =
            opengl::load_font_to_gpu(&vertices, &self.atlas_texture);

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo: None,
            tex_ids: vec![tex_id],
            primitives_len: vertices.len(),
            shader: ShaderType::TextShader,
        };

        self.to_opengl(&text, &gpu_bound);

        self.text_caching.insert(text.content.clone(), gpu_bound);
    }

    fn to_opengl(&self, text: &Text, gpu_bound: &GpuBound) {
        let Text {
            position,
            color,
            font_size,
            ..
        } = text;

        opengl::use_vao(gpu_bound.vao);
        opengl::bind_texture(gpu_bound.tex_ids[0], 0);

        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &position.to_glm());

        let size_ratio = font_size / self.size;

        let prog_id = SHADERS.get_program(ShaderType::TextShader);
        shaders::set_sampler(prog_id, 0);
        shaders::set_matrix4(prog_id, "model", model.as_slice());
        shaders::set_f32(prog_id, "font_size", size_ratio);
        shaders::set_vec3(prog_id, "text_color", &color.into());

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::DrawArrays(gl::TRIANGLES, 0, gpu_bound.primitives_len as i32);
            gl::Enable(gl::DEPTH_TEST);
        }
    }
}
