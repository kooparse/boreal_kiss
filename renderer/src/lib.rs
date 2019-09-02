mod object;
mod opengl;
pub mod primitives;
mod shaders;
mod texture;

use object::RendererObject;
use shaders::ShaderManager;
use shaders::ShaderType;
use std::collections::HashMap;
use std::ptr;

type LoadedObjectId = u64;
static mut LOADED_OBJECT_ID: LoadedObjectId = 0;

/// Define RGBA color.
/// (Sometime, tuple structs are not very elegent).
pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub struct RendererOptions {
    with_multisampling: bool,
    default_color: Color,
}

impl RendererOptions {
    pub fn new(with_multisampling: bool, default_color: Color) -> Self {
        Self {
            with_multisampling,
            default_color,
        }
    }
}

/// All the data needed to retrieve an object from the gpu memory.
struct GpuBound {
    #[allow(dead_code)]
    vbo: Option<u32>,
    texture_id: Option<u32>,
    ebo: Option<u32>,
    data_len: usize,
}

struct LoadedObject {
    #[allow(dead_code)]
    position: (f32, f32, f32),
    gpu_bound: GpuBound,
}

impl Drop for LoadedObject {
    fn drop(&mut self) {
        // TODO: clear object loaded in gpu mem.
    }
}

struct ObjectPool(HashMap<LoadedObjectId, LoadedObject>);

pub struct Renderer {
    #[allow(dead_code)]
    options: RendererOptions,
    object_pool: ObjectPool,
    render_group: HashMap<ShaderType, Vec<LoadedObjectId>>,
    shader_manager: ShaderManager,
}

impl Renderer {
    /// Create, compile and generate vertex array objects (vao) for our
    /// renderer.
    pub fn new(options: RendererOptions) -> Self {
        // Panic if opengl functions not loaded.
        opengl::get_opengl_loaded();

        opengl::set_multisampling(options.with_multisampling);
        opengl::clear_color(&options.default_color);

        // Compile all shaders and create corresponding vao.
        let shader_manager = ShaderManager::new();

        let mut render_group: HashMap<ShaderType, Vec<LoadedObjectId>> =
            HashMap::new();

        for key in shader_manager.list.keys() {
            render_group.insert(key.clone(), vec![]);
        }

        Self {
            options,
            object_pool: ObjectPool(HashMap::new()),
            shader_manager,
            render_group,
        }
    }

    /// We push objects into our render group and load data into gl.
    pub fn push<'t>(&mut self, objects: Vec<RendererObject<'t>>) {
        objects.into_iter().for_each(|mut object| {
            let program = &self.shader_manager.list[&object.shader_type];

            // If indices is set, we need to tell opengl to use
            // and generate ebo for this object.
            if !object.vertices.indices.is_empty() {
                let ebo = opengl::gen_buffer();
                object.vertices.ebo = Some(ebo);
            };

            // Load object to gpu (from system memmory).
            opengl::load_object_to_gpu(program.vao, &object);

            if let Some(values) = self.render_group.get_mut(&object.shader_type)
            {
                let gpu_bound = GpuBound {
                    ebo: object.vertices.ebo,
                    vbo: None,
                    data_len: if object.vertices.ebo.is_some() {
                        object.vertices.indices.len()
                    } else {
                        object.vertices.data.len()
                    },
                    texture_id: if let Some(texture) = &object.texture {
                        texture.id
                    } else {
                        None
                    },
                };

                let loaded_obj = LoadedObject {
                    position: (0.0, 0.0, 0.0),
                    gpu_bound,
                };

                unsafe {
                    LOADED_OBJECT_ID += 1;
                    self.object_pool.0.insert(LOADED_OBJECT_ID, loaded_obj);

                    values.push(LOADED_OBJECT_ID);
                }
            };
        });
    }

    pub fn draw(&mut self) {
        for (shader_type, ids) in &self.render_group {
            let program = &self.shader_manager.list[&shader_type];
            opengl::use_shader_program(program.program_id);
            opengl::use_vao(program.vao);

            ids.iter().for_each(|id| unsafe {
                if let Some(obj) = self.object_pool.0.get(id) {
                    let gpu_bound = &obj.gpu_bound;

                    if let Some(texture) = gpu_bound.texture_id {
                        gl::BindTexture(gl::TEXTURE_2D, texture);
                    }

                    if let Some(ebo) = gpu_bound.ebo {
                        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                        gl::DrawElements(
                            gl::TRIANGLES,
                            gpu_bound.data_len as i32,
                            gl::UNSIGNED_INT,
                            ptr::null(),
                        );
                    } else {
                        gl::DrawArrays(
                            gl::TRIANGLES,
                            0,
                            gpu_bound.data_len as i32,
                        );
                    }
                };
            });
        }

        // TODO: Used only for debugging...
        self.clear_all();
    }

    // TODO: This do not remove ids in the render group.
    pub fn remove_item(&mut self, id: LoadedObjectId) {
        self.object_pool.0.remove(&id);
    }

    /// The method shrink_to_fit will frees any allocated
    /// memory that is not used.
    pub fn clear_all(&mut self) {
        self.render_group.clear();
        self.render_group.shrink_to_fit();

        self.object_pool.0.clear();
        self.object_pool.0.shrink_to_fit();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.clear_all();
    }
}
