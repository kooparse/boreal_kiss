mod opengl;
pub mod primitives;
mod shaders;
mod texture;
mod vertex;

use nalgebra_glm as glm;
use opengl::{TexId, EBO, VAO, VBO};
use shaders::{ShaderManager, ShaderType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use texture::Texture;
use vertex::{Vector3, Vertex};

type LoadedObjectId = u64;
static mut LOADED_OBJECT_ID: LoadedObjectId = 0;

/// Define RGBA color.
/// (Sometime, tuple structs are not very elegent).
#[derive(Default)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

#[derive(Default)]
pub struct RendererOptions {
    with_multisampling: bool,
    with_depth_testing: bool,
    default_color: Color,
}

impl RendererOptions {
    pub fn new(
        with_multisampling: bool,
        with_depth_testing: bool,
        default_color: Color,
    ) -> Self {
        Self {
            with_multisampling,
            with_depth_testing,
            default_color,
        }
    }
}

/// All the data linked to our backend renderer.
struct GpuBound {
    vao: VAO,
    vbo: VBO,
    ebo: Option<EBO>,
    texture_id: Option<TexId>,
    primitives_len: usize,
    shader: ShaderType,
}

struct LoadedObject {
    #[allow(unused)]
    name: String,
    is_hidden: bool,
    position: Vector3,
    gpu_bound: GpuBound,
}

impl Drop for LoadedObject {
    fn drop(&mut self) {
        unsafe {
            // Delete VAO.
            gl::DeleteVertexArrays(1, [self.gpu_bound.vao].as_ptr());

            // Delete texture.
            if let Some(tex_id) = self.gpu_bound.texture_id {
                gl::DeleteTextures(1, [tex_id].as_ptr());
            }

            // Delete VBO and EBO.
            if let Some(ebo) = self.gpu_bound.ebo {
                gl::DeleteBuffers(2, [self.gpu_bound.vbo, ebo].as_ptr());
            } else {
                gl::DeleteBuffers(1, [self.gpu_bound.vbo].as_ptr());
            }
        }
    }
}

pub struct Mesh<'t, 'n> {
    pub name: &'n str,
    pub vertex: Vertex,
    pub texture: Option<Texture<'t>>,
    pub shader_type: ShaderType,
    pub position: glm::TVec3<f32>,
}

impl<'t, 'n> From<&Mesh<'t, 'n>> for LoadedObject {
    fn from(object: &Mesh<'t, 'n>) -> LoadedObject {
        // From system memmory to gpu memory.
        let (vao, vbo, ebo, texture_id) = opengl::load_object_to_gpu(&object);

        let primitives_len = ebo.map_or(object.vertex.primitives.len(), |_| {
            object.vertex.indices.len()
        });

        let gpu_bound = GpuBound {
            vao,
            vbo,
            ebo,
            primitives_len,
            shader: object.shader_type.clone(),
            texture_id,
        };

        LoadedObject {
            name: object.name.to_string(),
            is_hidden: false,
            position: object.position,
            gpu_bound,
        }
    }
}

pub struct RenderState {
    pub projection: glm::TMat4<f32>,
    pub view: glm::TMat4<f32>,
}

impl RenderState {
    pub fn new(projection: glm::TMat4<f32>, view: glm::TMat4<f32>) -> Self {
        Self { projection, view }
    }
}

pub struct Renderer<'r> {
    options: RendererOptions,
    object_storage: HashMap<LoadedObjectId, LoadedObject>,
    shader_manager: ShaderManager,
    state: &'r RefCell<RenderState>,
}

impl<'r> Renderer<'r> {
    /// Create, compile and generate vertex array objects (vao) for our
    /// renderer.
    pub fn new(
        options: RendererOptions,
        state: &'r RefCell<RenderState>,
    ) -> Self {
        // Panic if opengl functions not loaded.
        opengl::get_opengl_loaded();

        opengl::set_multisampling(options.with_multisampling);
        opengl::set_depth_testing(options.with_depth_testing);
        opengl::clear(&options.default_color);

        // Compile all shaders and create corresponding vao.
        let shader_manager = ShaderManager::new();

        Self {
            options,
            state,
            object_storage: HashMap::new(),
            shader_manager,
        }
    }

    /// TODO: Fix this function. The ratio isn't good. We should correct
    /// the aspect ratio on resize. It currently zoom in the matrix.
    pub fn update_viewport(&mut self, _width: f64, _height: f64, _dpi: f64) {
        // opengl::set_viewport(
        //     (width * dpi) as i32,
        //     (height * dpi) as i32,
        // );
    }

    /// We push objects into the storage and load data into gl.
    pub fn add_mesh(&mut self, object: Mesh) -> LoadedObjectId {
        unsafe {
            LOADED_OBJECT_ID += 1;
            self.object_storage
                .insert(LOADED_OBJECT_ID, LoadedObject::from(&object));

            LOADED_OBJECT_ID
        }
    }

    pub fn add_meshes(&mut self, objects: Vec<Mesh>) -> Vec<LoadedObjectId> {
        let mut ids = vec![];
        objects.iter().for_each(|object| unsafe {
            LOADED_OBJECT_ID += 1;
            ids.push(LOADED_OBJECT_ID);
            self.object_storage
                .insert(LOADED_OBJECT_ID, LoadedObject::from(object));
        });

        ids
    }

    /// Hide a mesh (it will still be loaded in the gpu mem).
    pub fn hide_mesh(&mut self, id: LoadedObjectId) {
        if let Some(object) = self.object_storage.get_mut(&id) {
            object.is_hidden = true;
        }
    }

    /// Show a hidden mesh.
    pub fn show_mesh(&mut self, id: LoadedObjectId) {
        if let Some(object) = self.object_storage.get_mut(&id) {
            object.is_hidden = false;
        }
    }

    /// Toggle show/hidden mesh.
    pub fn toggle_mesh(&mut self, id: LoadedObjectId) {
        if let Some(object) = self.object_storage.get_mut(&id) {
            if object.is_hidden {
                self.show_mesh(id);
            } else {
                self.hide_mesh(id);
            }
        }
    }

    pub fn draw(&mut self) {
        for obj in self.object_storage.values() {
            if obj.is_hidden {
                continue;
            }

            let state = self.state.borrow();
            let gpu_bound = &obj.gpu_bound;
            let program = &self.shader_manager.list[&gpu_bound.shader];

            opengl::use_shader_program(program.program_id);

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

            opengl::use_vao(gpu_bound.vao);

            let mut model = glm::Mat4::identity();
            model = glm::rotate(&model, -45.0, &glm::vec3(1.0, 0.0, 0.0));
            model = glm::translate(&model, &obj.position);

            shaders::set_matrix4(program.program_id, "model", model.as_slice());

            unsafe {
                if let Some(texture) = gpu_bound.texture_id {
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                }

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
        }
    }

    pub fn remove_mesh(&mut self, id: LoadedObjectId) {
        self.object_storage.remove(&id);
    }

    /// The method shrink_to_fit will frees any allocated
    /// memory that is not used.
    pub fn flush(&mut self) {
        self.object_storage.clear();
        self.object_storage.shrink_to_fit();
    }

    pub fn clear_screen(&self) {
        opengl::clear(&self.options.default_color);
    }
}

impl<'r> Drop for Renderer<'r> {
    fn drop(&mut self) {
        self.flush();
    }
}
