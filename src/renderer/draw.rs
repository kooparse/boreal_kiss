use super::{
    opengl,
    shaders::{self, ShaderType},
    Font, Mesh, Rgba, SunLight, Text, Transform, Vector,
};
// use crate::colliders::{BoundingBox, Collider};
use crate::global::*;
use crate::map::{Tile, Tilemap, World};
use crate::player::Player;
use crate::{Entities, Entity};
use nalgebra_glm as glm;
use std::ptr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points,
}

pub fn draw_world(entities: &Entities, player: &Player, world: &World) {
    for i in 0..TILEMAPS_COUNT.0 {
        for j in 0..TILEMAPS_COUNT.1 {
            let pos = glm::vec2(i as i32, j as i32);
            let world_tilemap = world.get_tilemap(&pos);

            if let Some(tilemap) = world_tilemap {
                let tilemap = entities.get(&tilemap);
                draw_tilemap(entities, player, world, tilemap, Some(&pos));
            }
        }
    }
}

pub fn draw_tilemap(
    entities: &Entities,
    player: &Player,
    world: &World,
    tilemap: &Tilemap,
    offset: Option<&glm::TVec2<i32>>,
) {
    for x in 0..TILES_COUNT.0 {
        for y in 0..TILES_COUNT.1 {
            let tile = tilemap.get_tile(x, y);

            let mut x = world.offset.x + x as f32 * TILE_SIZE;
            // Grid is in 2d, so "y" become "z" in 3d.
            let mut z = world.offset.y + y as f32 * TILE_SIZE;

            // Used for drawing the world map.
            if let Some(offset) = offset {
                x += offset.x as f32 * TILEMAP_WIDTH;
                z += offset.y as f32 * TILEMAP_HEIGHT;
            }

            let position = Transform::from_pos(Vector(x, 0., z));
            draw_tile(entities, player, &tile, &position);
        }
    }
}

pub fn draw_tile(
    entities: &Entities,
    player: &Player,
    tile: &Tile,
    position: &Transform,
) {
    // Refence of all Tile meshes.
    let markers = &entities
        .markers
        .as_ref()
        .expect("WTF, marker should be loaded before");

    // Always draw the ground (for now).
    let ground = entities.get(&markers.ground);
    draw_mesh(ground, None, position);

    // Match the tile type, and draw accordingly.
    // After, i should call func like "draw_player" or "draw_wall".
    match tile {
        Tile::Wall(handle) => {
            let wall = entities.get(handle);
            let mut transform = position.clone();
            transform.position = Vector(wall.world_pos.x, 1., wall.world_pos.y);
            draw_mesh(&entities.get(&markers.wall), None, &transform);
        }
        Tile::Player => {
            let mut transform = position.clone();
            // transform.position.1 = 1.;
            transform.position = Vector(
                player.world_pos.x * TILE_SIZE,
                1.,
                player.world_pos.z * TILE_SIZE,
            );
            draw_mesh(&entities.get(&markers.player), None, &transform);
        }
        _ => {}
    };
}

// obb
pub fn draw_bbox(entity: &Mesh, bbox: &Mesh) {
    if entity.collider.is_none() {
        return;
    }

    let prog_id = SHADERS.activate(ShaderType::SimpleShader);
    opengl::use_vao(bbox.gpu_bound.vao);

    let identity = glm::identity();
    let model = entity.transform.to_model();

    let size = bbox.bounding_box.max.to_glm();
    let center = bbox.bounding_box.center.to_glm();

    let bbox_model = glm::translate(&identity, &center)
        * glm::scale(&identity, &(size * 1.1));

    let model = model * bbox_model;
    shaders::set_matrix4(prog_id, "model", model.as_slice());

    // Set shader flags.
    bbox.flags.set_flags_to_shader(prog_id);
    let ebo = bbox.gpu_bound.ebo.unwrap();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::DrawElements(
            gl::TRIANGLES,
            bbox.gpu_bound.primitives_len as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );

        // Clear
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
}

pub fn draw_mesh(mesh: &Mesh, parent: Option<&Mesh>, position: &Transform) {
    if mesh.is_hidden {
        return;
    }

    let gpu_bound = &mesh.gpu_bound;

    let prog_id = SHADERS.activate(ShaderType::SimpleShader);
    opengl::use_vao(gpu_bound.vao);

    shaders::set_bool(prog_id, "is_hover", mesh.is_hover);
    if mesh.is_hover {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    let mut entity_model = position.to_model();
    // Perform parent transform to child.
    if let Some(parent) = parent {
        entity_model = parent.transform.to_model() * entity_model;
    }

    shaders::set_matrix4(prog_id, "model", entity_model.as_slice());

    // Set shader flags.
    mesh.flags.set_flags_to_shader(prog_id);

    gpu_bound
        .tex_ids
        .iter()
        .enumerate()
        .for_each(|(index, tex_id)| {
            shaders::set_sampler(prog_id, index);
            opengl::bind_texture(*tex_id, index);
        });

    unsafe {
        match mesh.mode {
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
                gl::DrawArrays(gl::LINES, 0, gpu_bound.primitives_len as i32);
            }

            _ => unimplemented!(),
        }
        gl::Disable(gl::BLEND);
    }
}

// Draw directional sun light over the scene.
pub fn draw_sun_light(_light_source: &SunLight) {
    // draw stuff.
}

// Render some text to the screen.
// Used only for the editor/UI for now.
pub fn draw_text(font: &mut Font, text: &Text) {
    // Activate the text shader.
    let prog_id = SHADERS.activate(ShaderType::TextShader);
    opengl::use_shader_program(prog_id);

    shaders::set_matrix4(
        prog_id,
        "projection",
        ORTHO_MATRIX.lock().unwrap().as_slice(),
    );

    font.render(text);
}
