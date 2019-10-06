use crate::{
    opengl,
    shaders::{self, ShaderProgramId, ShaderType},
    texture::Texture,
    utils, GpuBound, RenderState, Text,
};
use image::{DynamicImage, Luma};
use nalgebra_glm as glm;
use rusttype::{point, FontCollection, HMetrics, Rect, Scale};
use std::{collections::HashMap, fs::File, io::Read};

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzéèôçñ";
const ALPHA_UP: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÉÈÔÇÑ";
const NUM: &str = "1234567890";
const SPECIALS: &str = "!?.,:;'(){}[]/+|_-\"\\ ";

#[derive(Debug)]
pub struct Character {
    letter: String,
    h_metrics: HMetrics,
    bounding_box: Rect<i32>,
    texture: Texture,
    gpu_bound: GpuBound,
}

pub struct Font {
    charachers: HashMap<String, Character>,
    space: f32,
    pub name: String,
}

impl Font {
    pub fn new(font_path: &str) -> Self {
        let mut font_data = vec![];
        let mut file =
            File::open(font_path).expect("Error while openning the font.");

        let name: String = font_path
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .map(|n| n.to_string())
            .expect("Error while retrieving the font name.");

        file.read_to_end(&mut font_data)
            .expect("Error while reading the font.");

        let collection = FontCollection::from_bytes(&font_data)
            .expect("Error while constructing the font collection.");

        let font = collection
            .into_font()
            .expect("Error while returning a proper font.");

        let chars = [ALPHA, ALPHA_UP, NUM, SPECIALS].join("");

        let scale = Scale::uniform(21.);
        let v_metrics = font.v_metrics(scale);
        let offset = point(0., v_metrics.ascent);

        let glyphs: Vec<_> = font.layout(&chars, scale, offset).collect();

        let mut charachers = HashMap::new();
        let mut space: f32 = 0.;

        glyphs.iter().enumerate().for_each(|(index, glyph)| {
            let letters: Vec<&str> = chars.split("").collect();

            let font = glyph.font().unwrap();
            let v_metrics = font.v_metrics(scale);
            let h_metrics = glyph.unpositioned().h_metrics();

            if glyph.pixel_bounding_box().is_none() {
                space = h_metrics.advance_width;
            }

            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                let width = bounding_box.width() as u32;
                let height =
                    (v_metrics.ascent - v_metrics.descent).ceil() as u32;

                let mut img = DynamicImage::new_luma8(width, height).to_luma();

                glyph.draw(|x, y, alpha| {
                    img.put_pixel(
                        x,
                        y + bounding_box.min.y as u32,
                        Luma([(alpha * 255.) as _]),
                    )
                });

                // First character is escape.
                let letter = letters[index + 1].to_owned();

                #[cfg_attr(rustfmt, rustfmt_skip)]
                let vertices = [
                    0., height as f32, 0.0, 0.0,
                    0.,  0., 0.0, 1.0,
                    width as f32, 0., 1.0, 1.0,

                    0., height as f32, 0.0, 0.0,
                    width as f32, 0., 1.0, 1.0,
                    width as f32, height as f32, 1.0, 0.0,
                ];

                let texture = Texture::new((width, height), img.into_vec());

                // Generate the Quad.
                let (vao, vbo, tex_id) =
                    opengl::load_font_to_gpu(&vertices, &texture);

                let gpu_bound = GpuBound {
                    vao,
                    vbo,
                    ebo: None,
                    tex_ids: vec![tex_id],
                    primitives_len: vertices.len(),
                    shader: ShaderType::TextShader,
                };

                let character = Character {
                    letter: letter.clone(),
                    texture,
                    h_metrics,
                    bounding_box,
                    gpu_bound,
                };

                charachers.insert(letter, character);
            };
        });

        Self {
            charachers,
            space,
            name,
        }
    }

    /// This won't return the entire gpu_bound, but only what's
    /// necessary. We won't clone the gpu_bound, because it would
    /// call a drop... and clear vao/vbo on opengl.
    pub fn render(
        &self,
        text: &Text,
        program_id: &ShaderProgramId,
        r_state: &RenderState,
    ) {
        let content: Vec<&str> = text.content.split("").collect();
        let mut advance = 0.;
        let padding = 0.;

        shaders::set_matrix4(
            *program_id,
            "projection",
            utils::ortho_proj(r_state).as_slice(),
        );

        // Activate blend mode for the transparency.
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        content.into_iter().for_each(|letter| {
            if let Some(character) = self.charachers.get(letter) {
                let x = text.position.0
                    + character.h_metrics.left_side_bearing
                    + advance
                    + padding;
                let y = text.position.1 + padding;

                let vao = character.gpu_bound.vao;
                let tex_id = character.gpu_bound.tex_ids[0];
                let len = character.gpu_bound.primitives_len;

                opengl::use_vao(vao);

                // We're going to have only one texture (the one with the letter),
                // so it's cool to hardcoded the sampler number. Same for
                // the texture.
                shaders::set_sampler(*program_id, 0);
                opengl::bind_texture(tex_id, 0);

                shaders::set_vec3(
                    *program_id,
                    "text_color",
                    &[text.color.r, text.color.g, text.color.b]
                );

                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &glm::vec3(x, y, 0.));

                shaders::set_matrix4(*program_id, "model", model.as_slice());

                unsafe {
                    gl::DrawArrays(gl::TRIANGLES, 0, len as i32);
                }

                advance += character.h_metrics.advance_width;
            } else {
                advance += self.space;
            }
        });
    }
}
