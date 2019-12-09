mod draw;
mod font;
mod light;
mod mesh;
mod opengl;
pub mod primitives;
mod shaders;
mod text;
mod texture;
mod types;

// Internal...
use crate::entities::{Entities, Entity};
use crate::global::*;
use crate::map::Map;
use draw::*;
use font::Font;
// Pub
pub use light::{LightProbes, SunLight};
pub use mesh::{Mesh, Transform, Vertex};
pub use opengl::GpuBound;
pub use shaders::ShaderManager;
pub use text::Text;
pub use types::{Rgb, Rgba, Vector};

#[derive(Default)]
pub struct DebugInfo {
    pub draw_call: u32,
    pub gpu_loaded_size: u32,
    pub is_wireframe: bool,
}

pub struct Renderer {
    back_buffer_color: Rgba,
    font: Font,
    pub debug_info: DebugInfo,
}

impl Renderer {
    /// Create, compile and generate vertex array objects (vao) for our
    /// renderer.
    pub fn new(back_buffer_color: Rgba) -> Self {
        // Panic if opengl functions not loaded.
        // Display OpenGL version on the console.
        opengl::get_opengl_loaded();

        opengl::set_multisampling(true);
        opengl::set_depth_testing(true);

        // First paint the back_buffer in the default color.
        opengl::clear(&back_buffer_color);

        let font = Font::new(
            "assets/fonts/Helvetica/helvetica.json",
            "assets/fonts/Helvetica/helvetica.png",
        );

        Self {
            back_buffer_color,
            debug_info: DebugInfo::default(),
            font,
        }
    }

    pub fn draw(&mut self, entities: &mut Entities, map: &Map) {
        let bbox_mesh = primitives::create_cube(
            Transform::default(),
            None,
            Rgba::default(),
        );

        // Updates UBOs...
        SHADERS.update_all_ubo();

        // Reset the debug counter.
        self.debug_info.draw_call = 0;

        for i in 0..10 {
            for j in 0..10 {
                let value = map.grid[i][j];

                let x = i as f32 * 2.;
                let z = j as f32 * 2.;
                let mut transform = Transform::from_pos(Vector(x, 0., z));

                if value == 0 {
                    let color = Rgba::new(0., 1., 1., 1.);
                    draw_mesh(&primitives::create_tiles(transform, color), None);
                } else if value == 1 {
                    let color = Rgba::new(0., 1., 0., 1.);
                    transform.position.1 = 1.;
                    draw_mesh(
                        &primitives::create_cube(transform, None, color),
                        None,
                    );
                } else if value == 2 {
                    let color = Rgba::new(1., 1., 0., 1.);
                    transform.position.1 = 1.;
                    draw_mesh(
                        &primitives::create_cube(transform, None, color),
                        None,
                    );
                } else if value == 3 {
                    let color = Rgba::new(1., 0., 0., 1.);
                    transform.position.1 = 1.;
                    draw_mesh(
                        &primitives::create_cube(transform, None, color),
                        None,
                    );
                } else if value == 4 {
                    let color = Rgba::new(1., 0., 1., 1.);
                    let p_color = Rgba::new(0., 1., 1., 1.);
                    draw_mesh(&primitives::create_tiles(transform, p_color), None);
                    transform.position.1 = 0.5;
                    transform.scale = Vector(0.5, 0.5, 0.5);
                    draw_mesh(
                        &primitives::create_cube(transform, None, color),
                        None,
                    );
                }
            }
        }

        // Render all our meshes to the screen.
        for (mesh, _) in entities.meshes.iter() {
            self.debug_info.draw_call += 1;

            let parent = mesh.parent.as_ref().map(|p| entities.get(&p));

            if mesh.is_selected {
                draw_bbox(mesh, &bbox_mesh);
            }

            draw_mesh(mesh, parent);
        }

        // Render all our light probes into the scene.
        for (light, _) in entities.light_probes.iter() {
            self.debug_info.draw_call += 1;
            match light {
                LightProbes::Sun(sun) => draw_sun_light(sun),
            }
        }

        // Render all our GUI texts to the screen.
        for (text, _) in entities.text_widgets.iter() {
            self.debug_info.draw_call += 1;
            draw_text(&mut self.font, text);
        }

        // bbox goes out of scope so drop so gl cleanup functions are called.
    }

    pub fn clear_screen(&self) {
        opengl::clear(&self.back_buffer_color);
    }

    pub fn toggle_wireframe(&mut self) {
        unsafe {
            if self.debug_info.is_wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                self.debug_info.is_wireframe = false;
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                self.debug_info.is_wireframe = true;
            }
        }
    }
}
