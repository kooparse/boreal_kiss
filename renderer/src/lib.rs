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

use crate::font::Font;
pub use crate::storage::{GeneratedId, Storage};
use crate::text::Text;
use nalgebra_glm as glm;
use opengl::{TexId, EBO, VAO, VBO};
use ray::Ray;
use shaders::{ShaderFlags, ShaderManager, ShaderType};
use std::cmp::min;
use std::ptr;
use texture::Texture;
use vertex::{Vector3, Vertex};

#[derive(Default)]
pub struct Pos2D(pub f32, pub f32);
#[derive(Default)]
pub struct Pos3D(pub f32, pub f32, pub f32);

/// Define RGBA color.
/// (Sometime, tuple structs are not very elegent).
#[derive(Default)]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

/// Define RGB color.
pub struct Rgb(pub f32, pub f32, pub f32);

impl Default for Rgb {
    fn default() -> Self {
        Self(255., 255., 255.)
    }
}

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

/// All the data linked to our backend renderer.
#[derive(Debug)]
pub struct GpuBound {
    vao: VAO,
    vbo: VBO,
    ebo: Option<EBO>,
    tex_ids: Vec<TexId>,
    primitives_len: usize,
    shader: ShaderType,
}

impl Drop for GpuBound {
    fn drop(&mut self) {
        unsafe {
            // Delete VAO.
            gl::DeleteVertexArrays(1, [self.vao].as_ptr());

            // Delete texture.
            gl::DeleteTextures(
                self.tex_ids.len() as i32,
                self.tex_ids.as_ptr(),
            );

            // Delete VBO and EBO.
            if let Some(ebo) = self.ebo {
                gl::DeleteBuffers(2, [self.vbo, ebo].as_ptr());
            } else {
                gl::DeleteBuffers(1, [self.vbo].as_ptr());
            }
        }
    }
}

pub struct LoadedObject {
    #[allow(unused)]
    name: String,
    is_hidden: bool,
    world_pos: Vector3,
    mode: DrawMode,
    gpu_bound: GpuBound,
    flags: ShaderFlags,
}

pub struct Mesh<'n> {
    pub name: &'n str,
    pub vertex: Vertex,
    pub textures: Vec<Texture>,
    pub shader_type: ShaderType,
    pub world_pos: glm::TVec3<f32>,
    pub mode: DrawMode,
}

impl<'n> From<&Mesh<'n>> for LoadedObject {
    fn from(object: &Mesh<'n>) -> LoadedObject {
        // From system memmory to gpu memory.
        let (vao, vbo, ebo, tex_ids) = opengl::load_object_to_gpu(&object);

        let primitives_len = ebo.map_or(object.vertex.primitives.len(), |_| {
            object.vertex.indices.len()
        });

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo,
            primitives_len,
            shader: object.shader_type.clone(),
            tex_ids,
        };

        let (has_uv, has_multi_uv, has_vert_colors, _tex_number) = {
            let colors = &object.vertex.colors;
            // We want a correlation between the number of set of coords
            // and the number of texture loaded.
            let tex_number =
                min(object.vertex.uv_coords.len(), object.textures.len());

            (
                tex_number > 0,
                tex_number > 1,
                !colors.is_empty(),
                tex_number,
            )
        };

        let flags = ShaderFlags {
            has_uv,
            has_multi_uv,
            has_vert_colors,
        };

        LoadedObject {
            name: object.name.to_string(),
            is_hidden: false,
            mode: object.mode,
            world_pos: object.world_pos,
            gpu_bound,
            flags,
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
    pub mesh_call_nb: u32,
    pub text_call_nb: u32,
    pub gpu_loaded_size: u32,
}

pub struct Renderer {
    // Some options like resolution, etc...
    options: RendererOptions,
    // Store all our mesh there (only the gpu information).
    object_storage: Storage<LoadedObject>,
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

        Self {
            options,
            object_storage: Storage::default(),
            default_font: Font::new("assets/fonts/Roboto/Roboto-Regular.ttf"),
            text_storage: Storage::default(),
            shader_manager,
            debug_info: DebugInfo::default(),
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
    pub fn add_mesh(&mut self, object: Mesh) -> GeneratedId {
        self.object_storage.push(LoadedObject::from(&object))
    }
    pub fn add_meshes(&mut self, objects: Vec<Mesh>) -> Vec<GeneratedId> {
        objects
            .iter()
            .map(|object| self.object_storage.push(LoadedObject::from(object)))
            .collect()
    }

    /// Hide a mesh (it will still be loaded in the gpu mem).
    pub fn hide_mesh(&mut self, id: GeneratedId) {
        if let Some(object) = self.object_storage.get_mut(&id) {
            object.is_hidden = true;
        }
    }

    /// Show a hidden mesh.
    pub fn show_mesh(&mut self, id: GeneratedId) {
        if let Some(object) = self.object_storage.get_mut(&id) {
            object.is_hidden = false;
        }
    }

    /// Toggle show/hidden mesh.
    pub fn toggle_mesh(&mut self, id: GeneratedId) {
        if let Some(object) = self.object_storage.get_mut(&id) {
            if object.is_hidden {
                self.show_mesh(id);
            } else {
                self.hide_mesh(id);
            }
        }
    }

    pub fn add_ray(
        &mut self,
        origin: glm::TVec3<f32>,
        direction: glm::TVec3<f32>,
        length: f32,
    ) {
        let ray = Ray::new(origin, direction, length);
        let name = format!("ray: {}", direction);
        self.add_mesh(primitives::create_line(&name, ray));
    }

    pub fn add_text<T: ToString>(
        &mut self,
        text: T,
        position: Pos2D,
        color: Rgb,
    ) -> GeneratedId {
        let text = Text {
            content: text.to_string(),
            position,
            font_attached: "Roboto-Regular.ttf".to_owned(),
            color,
        };

        self.text_storage.push(text)
    }

    pub fn update_text<T: ToString>(
        &mut self,
        text_to_replace: GeneratedId,
        text: T,
        position: Pos2D

    ) {
        if let Some(to_replace) = self.text_storage.get_mut(&text_to_replace) {
            to_replace.content = text.to_string();
            to_replace.position = position;
        }
    }


    pub fn remove_text(&mut self, id: &GeneratedId) {
        self.text_storage.remove(id);
    }

    pub fn draw(&mut self, state: &RenderState) {
        self.debug_info.mesh_call_nb = 0;

        // Render all our meshes to the screen.
        for obj in self.object_storage.items.values() {
            if obj.is_hidden {
                continue;
            }

            let gpu_bound = &obj.gpu_bound;
            let program = &self.shader_manager.list[&gpu_bound.shader];

            opengl::use_shader_program(program.program_id);
            opengl::use_vao(gpu_bound.vao);

            shaders::set_matrix4(
                program.program_id,
                "view",
                state.view.as_slice(),
            );

            // TODO: Don't set projection matrix in the render loop.
            shaders::set_matrix4(
                program.program_id,
                "projection",
                state.projection.as_slice(),
            );

            // TODO: This should be (maybe) stored in the object.
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &obj.world_pos);

            shaders::set_matrix4(program.program_id, "model", model.as_slice());

            // Set shader flags.
            obj.flags.set_flags_to_shader(program.program_id);

            gpu_bound
                .tex_ids
                .iter()
                .enumerate()
                .for_each(|(index, tex_id)| {
                    shaders::set_sampler(program.program_id, index);
                    opengl::bind_texture(*tex_id, index);
                });

            unsafe {
                gl::Disable(gl::BLEND);
                self.debug_info.mesh_call_nb += 1;

                match obj.mode {
                    DrawMode::Triangles => {
                        if let Some(ebo) = gpu_bound.ebo {
                            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                            gl::DrawElements(
                                gl::TRIANGLES,
                                gpu_bound.primitives_len as i32,
                                gl::UNSIGNED_INT,
                                ptr::null(),
                            );
                        } else {
                            gl::DrawArrays(
                                gl::TRIANGLES,
                                0,
                                gpu_bound.primitives_len as i32,
                            );
                        }
                    }

                    DrawMode::Lines => {
                        gl::DrawArrays(
                            gl::LINES,
                            0,
                            gpu_bound.primitives_len as i32,
                        );
                    }

                    _ => unimplemented!(),
                }
            }
        }

        // Activate the text shader.
        let program = &self.shader_manager.list[&ShaderType::TextShader];
        opengl::use_shader_program(program.program_id);

        for text in self.text_storage.items.values() {
            self.default_font.render(text, &program.program_id, &state);
        }
    }

    pub fn remove_mesh(&mut self, id: GeneratedId) {
        self.object_storage.remove(&id);
    }

    /// The method shrink_to_fit will frees any allocated
    /// memory that is not used.
    pub fn flush_meshes(&mut self) {
        self.object_storage.clear();
    }

    pub fn clear_screen(&self) {
        opengl::clear(&self.options.default_color);
    }
}
