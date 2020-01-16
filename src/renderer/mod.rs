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
use crate::entities::{Entities, Entity, Handle, Markers};
use crate::global::*;
use crate::player::Player;
use crate::tilemap::{Tile, World};
use draw::*;
use font::Font;
// Pub
pub use light::{LightProbes, SunLight};
pub use mesh::{Mesh, Transform, Vertex};
pub use opengl::GpuBound;
pub use shaders::ShaderManager;
pub use text::Text;
pub use types::{Colors, Rgb, Rgba, Vector};

use nalgebra_glm as glm;

struct MarkedHandle {
    ground: Handle<Mesh>,
    wall: Handle<Mesh>,
}

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
    pub fn new(back_buffer_color: Rgba, entities: &mut Entities) -> Self {
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

        // Load mesh assets.
        let ground = entities.insert(primitives::create_tiles(
            "assets/textures/ground.png",
            Transform::default(),
            Rgba::new(0., 1., 1., 1.),
        ));
        let wall = entities.insert(primitives::create_cube(
            "",
            Transform::default(),
            None,
            Rgba::default(),
        ));

        let player = entities.insert(primitives::create_cube(
            "assets/textures/player.png",
            Transform::default(),
            None,
            Rgba::red(),
        ));

        entities.markers = Some(Markers {
            ground,
            wall,
            player,
        });

        Self {
            back_buffer_color,
            debug_info: DebugInfo::default(),
            font,
        }
    }

    pub fn draw(
        &mut self,
        entities: &mut Entities,
        world: &World,
        player: &Player,
    ) {
        // let bbox_mesh = primitives::create_cube(
        //     Transform::default(),
        //     None,
        //     Rgba::default(),
        // );

        // Updates UBOs...
        SHADERS.update_all_ubo();

        // Reset the debug counter.
        self.debug_info.draw_call = 0;

        // Draw current tilemap.
        let tilemap = entities.get(&player.tilemap_pos.handle);
        dbg!(&player.tilemap_pos);
        draw_tilemap(
            entities,
            player,
            world,
            tilemap,
            Some(&player.tilemap_pos.world),
        );


        for (handle, pos) in
            world.get_sibling_tilemap(&player.tilemap_pos.world)
        {
            // Render the "current" tilemap.
            let tilemap = entities.get(&handle);
            draw_tilemap(entities, player, world, tilemap, Some(&pos));
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
