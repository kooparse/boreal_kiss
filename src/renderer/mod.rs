mod font;
mod opengl;
pub mod primitives;
mod ray;
mod shaders;
mod storage;
mod text;
mod texture;
mod mesh;
mod types;
mod light;
mod draw;

// Internal...
use nalgebra_glm as glm;
use mesh::LoadedMesh;
use font::Font;
use storage::{Storage};
use draw::*;
// Pub
pub use light::{LightSource, SunLight};
pub use mesh::Mesh;
pub use opengl::GpuBound;
pub use types::{Rgba,Rgb, Vector};
pub use storage::GenerationId;
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
    mesh_storage: Storage<LoadedMesh>,
    // For now, only one font...
    default_font: Font,
    // Store all the text that should be rendered on the screen.
    text_storage: Storage<Text>,
    light_storage: Storage<SunLight>,

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


        let default_font = Font::new(
            "assets/fonts/Helvetica/helvetica.json",
            "assets/fonts/Helvetica/helvetica.png",
        );

        Self {
            back_buffer_color,
            mesh_storage: Storage::default(),
            text_storage: Storage::default(),
            light_storage: Storage::default(),
            debug_info: DebugInfo::default(),
            default_font, 
        }
    }

    pub fn draw(&mut self) {
        // Reset the debug counter.
        self.debug_info.draw_call = 0;
        //
        // Render all our meshes to the screen.
        for mesh in self.mesh_storage.items.values() {
            self.debug_info.draw_call +=  1;
            draw_mesh(mesh);
        }

        for sun_light in self.light_storage.items.values() {
            draw_sun_light(sun_light);
        }
        //
        // Render all our texts to the screen.
        for text in self.text_storage.items.values_mut() {
            self.debug_info.draw_call +=  1;
            draw_text(&mut self.default_font, text);
        }
    }

    pub fn add_meshes(&mut self, objects: Vec<Mesh>) -> Vec<GenerationId> {
        objects
            .iter()
            .map(|object| self.mesh_storage.push(LoadedMesh::from(object)))
            .collect()
    }

    pub fn add_text(
        &mut self,
        text: Text,
    ) -> GenerationId {
        self.text_storage.push(text)
    }

    pub fn update_text(
        &mut self,
        text_id: GenerationId,
    ) -> &mut Text {
        if let Some(to_replace) = self.text_storage.get_mut(text_id) {
            to_replace
        } else {
            unimplemented!();
        }
    }

    // pub fn remove_text(&mut self, id: GenerationId) {
    //     self.text_storage.remove(id);
    // }

    // pub fn remove_mesh(&mut self, id: GenerationId) {
    //     self.mesh_storage.remove(id);
    // }

    /// The method shrink_to_fit will frees any allocated
    /// memory that is not used.
    pub fn flush_meshes(&mut self) {
        self.mesh_storage.clear();
    }

    pub fn clear_screen(&self) {
        opengl::clear(&self.back_buffer_color);
    }

    // pub fn add_ray(
    //     &mut self,
    //     origin: Vector,
    //     direction: Vector,
    //     length: f32,
    // ) {
    //     let ray = Ray::new(origin, direction, length);
    //     let name = format!("ray: {:?}", direction);
    //     self.add_mesh(primitives::create_line(&name, ray));
    // }

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
