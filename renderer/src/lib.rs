mod font;
mod opengl;
pub mod primitives;
mod ray;
mod shaders;
mod storage;
mod text;
mod texture;
mod utils;
mod vertex;
mod mesh;
mod color;
mod position;


use nalgebra_glm as glm;
use crate::{
    mesh::LoadedMesh,
    font::Font,
    storage::{Storage},
    ray::Ray,
    shaders::{ShaderManager, ShaderType, ShaderProgramId}
};
pub use crate::{
    position::Vector,
    mesh::Mesh,
    opengl::GpuBound,
    color::{Rgba,Rgb},
    storage::GenerationId,
    text::Text
};

#[derive(Debug)]
pub struct GameResolution {
    pub width: f64,
    pub height: f64,
    pub dpi: f64,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points,
}

pub enum ObjectType {
    Text,
    Mesh,
    Ray,
}

pub trait DrawableObject {
    fn draw(&self, state: &RenderState, program_id: ShaderProgramId);
    fn cleanup(&self);
}

#[derive(Default)]
pub struct RendererOptions {
    with_multisampling: bool,
    with_depth_testing: bool,
    default_color: Rgba,
}

impl RendererOptions {
    pub fn new(
        with_multisampling: bool,
        with_depth_testing: bool,
        default_color: Rgba,
    ) -> Self {
        Self {
            with_multisampling,
            with_depth_testing,
            default_color,
        }
    }
}

pub struct RenderState {
    pub resolution: GameResolution,
    pub projection: glm::TMat4<f32>,
    pub view: glm::TMat4<f32>,
}

impl RenderState {
    pub fn new(
        projection: glm::TMat4<f32>,
        view: glm::TMat4<f32>,
        resolution: GameResolution,
    ) -> Self {
        Self {
            projection,
            view,
            resolution,
        }
    }
}

#[derive(Default)]
pub struct DebugInfo {
    pub draw_call: u32,
    pub gpu_loaded_size: u32,
    pub is_wireframe: bool,
}


pub struct Renderer {
    // Some options like resolution, etc...
    options: RendererOptions,
    // Store all our mesh there (only the gpu information).
    mesh_storage: Storage<LoadedMesh>,
    // For now, only one font...
    default_font: Font,
    // Store all the text that should be rendered on the screen.
    text_storage: Storage<Text>,
    // Store all our shaders here.
    shader_manager: ShaderManager,

    pub debug_info: DebugInfo,
}

impl Renderer {
    /// Create, compile and generate vertex array objects (vao) for our
    /// renderer.
    pub fn new(options: RendererOptions) -> Self {
        // Panic if opengl functions not loaded.
        opengl::get_opengl_loaded();

        opengl::set_multisampling(options.with_multisampling);
        opengl::set_depth_testing(options.with_depth_testing);
        opengl::clear(&options.default_color);

        // Compile all shaders and create corresponding vao.
        let shader_manager = ShaderManager::new();

        let default_font = Font::new(
            "assets/fonts/Helvetica/helvetica.json",
            "assets/fonts/Helvetica/helvetica.png",
        );

        Self {
            options,
            mesh_storage: Storage::default(),
            text_storage: Storage::default(),
            debug_info: DebugInfo::default(),
            shader_manager,
            default_font, 
        }
    }

    pub fn draw(&mut self, state: &RenderState) {
        // Reset the debug counter.
        self.debug_info.draw_call = 0;
        //
        // Render all our meshes to the screen.
        for mesh in self.mesh_storage.items.values() {
            self.debug_info.draw_call +=  1;

            let program = &self.shader_manager.list[&mesh.gpu_bound.shader];
            mesh.draw(state, program.program_id);
            mesh.cleanup();
        }

        //
        // Render all our texts to the screen.
        for text in self.text_storage.items.values_mut() {
            self.debug_info.draw_call +=  1;

            // Activate the text shader.
            let program = &self.shader_manager.list[&ShaderType::TextShader];
            opengl::use_shader_program(program.program_id);

            shaders::set_matrix4(
                program.program_id,
                "projection",
                utils::ortho_proj(state).as_slice(),
            );

            self.default_font.render(text, program.program_id);
        }
    }

    /// TODO: Fix this function. The ratio isn't good. We should correct
    /// the aspect ratio on resize. It currently zoom in the matrix.
    pub fn update_viewport(&mut self, _resolution: &GameResolution) {
        // opengl::set_viewport(
        //     (width * dpi) as i32,
        //     (height * dpi) as i32,
        // );
    }

    /// We push objects into the storage and load data into gl.
    pub fn add_mesh(&mut self, object: Mesh) -> GenerationId {
        self.mesh_storage.push(LoadedMesh::from(&object))
    }
    pub fn add_meshes(&mut self, objects: Vec<Mesh>) -> Vec<GenerationId> {
        objects
            .iter()
            .map(|object| self.mesh_storage.push(LoadedMesh::from(object)))
            .collect()
    }

    /// Hide a mesh (it will still be loaded in the gpu mem).
    pub fn hide_mesh(&mut self, id: GenerationId) {
        if let Some(object) = self.mesh_storage.get_mut(id) {
            object.is_hidden = true;
        }
    }

    /// Show a hidden mesh.
    pub fn show_mesh(&mut self, id: GenerationId) {
        if let Some(object) = self.mesh_storage.get_mut(id) {
            object.is_hidden = false;
        }
    }

    /// Toggle show/hidden mesh.
    pub fn toggle_mesh(&mut self, id: GenerationId) {
        if let Some(object) = self.mesh_storage.get_mut(id) {
            if object.is_hidden {
                self.show_mesh(id);
            } else {
                self.hide_mesh(id);
            }
        }
    }

    pub fn add_text(
        &mut self,
        text: Text,
    ) -> GenerationId {
        // let loaded = LoadedText::from(text);
        self.text_storage.push(text)
    }

    pub fn update_text(
        &mut self,
        text_id: GenerationId,
    ) -> &mut Text {
        if let Some(to_replace) = self.text_storage.get_mut(text_id) {
            to_replace
        } else {
            panic!();
        }
    }

    pub fn remove_text(&mut self, id: GenerationId) {
        self.text_storage.remove(id);
    }

    pub fn remove_mesh(&mut self, id: GenerationId) {
        self.mesh_storage.remove(id);
    }

    /// The method shrink_to_fit will frees any allocated
    /// memory that is not used.
    pub fn flush_meshes(&mut self) {
        self.mesh_storage.clear();
    }

    pub fn clear_screen(&self) {
        opengl::clear(&self.options.default_color);
    }

    pub fn add_ray(
        &mut self,
        origin: Vector,
        direction: Vector,
        length: f32,
    ) {
        let ray = Ray::new(origin, direction, length);
        let name = format!("ray: {:?}", direction);
        self.add_mesh(primitives::create_line(&name, ray));
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
