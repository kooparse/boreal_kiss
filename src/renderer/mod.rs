mod font;
mod opengl;
pub mod primitives;
mod ray;
mod shaders;
mod text;
mod texture;
mod mesh;
mod types;
mod light;
mod draw;

// Internal...
use nalgebra_glm as glm;
use font::Font;
use draw::*;
use crate::entities::Entities;
// Pub
pub use light::{LightProbes, SunLight};
pub use mesh::{Mesh, LoadedMesh};
pub use opengl::GpuBound;
pub use types::{Rgba,Rgb, Vector};
pub use text::Text;
pub use shaders::ShaderManager;

pub struct SpaceTransform {
    // Orthographic matrix.
    gui: glm::Mat4,
    // Perspective matrix.
    projection: glm::Mat4,
    view: glm::Mat4,
}

#[derive(Default)]
pub struct DebugInfo {
    pub draw_call: u32,
    pub gpu_loaded_size: u32,
    pub is_wireframe: bool,
}


pub struct Renderer {
    // Some options like resolution, etc...
    back_buffer_color: Rgba,
    // Store all our mesh there (only the gpu information).
    // mesh_storage: Storage<LoadedMesh>,
    // For now, only one font...
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

        // let proj = glm::perspective::<f32>(
        //     1200. / 800.,
        //     45.0,
        //     0.1,
        //     100.0,
        // );

        // We want 3 mat4 (3 * 64 bytes).
        let ubo = opengl::generate_ubo(64, 0);
        // opengl::set_ubo(ubo, 0, proj);


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

    pub fn draw(&mut self, entities: &Entities) {
        // Reset the debug counter.
        self.debug_info.draw_call = 0;

        // Render all our meshes to the screen.
        for mesh in entities.meshes.iter() {
            self.debug_info.draw_call +=  1;
            draw_mesh(mesh);
        }

        // Render all our light probes into the scene.
        for light in entities.light_probes.iter() {
            self.debug_info.draw_call +=  1;
            match light {
                LightProbes::Sun(sun) => draw_sun_light(sun),
            }
        }

        // Render all our GUI texts to the screen.
        for text in entities.text_widgets.iter() {
            self.debug_info.draw_call +=  1;
            draw_text(&mut self.font, text);
        }
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
