use crate::entities::{Entities, Entity, Handle};
use crate::global::{TILEMAPS_COUNT, TILES_COUNT};
use crate::player::Player;
use crate::wall::Wall;
use nalgebra_glm as glm;
use std::ops::Add;

/// Create the game world.
pub fn init_world_and_player(entities: &mut Entities) -> (World, Player) {
    // Macro representation of the world.
    let mut world_grid = [
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
    ];

    // Place tile maps handles in the world.
    world_grid[0][0] = Some(entities.insert(Tilemap::default()));
    world_grid[0][1] = Some(entities.insert(Tilemap::default()));
    world_grid[0][2] = Some(entities.insert(Tilemap::default()));
    world_grid[0][3] = Some(entities.insert(Tilemap::default()));
    world_grid[0][4] = Some(entities.insert(Tilemap::default()));
    world_grid[1][3] = Some(entities.insert(Tilemap::default()));
    world_grid[2][3] = Some(entities.insert(Tilemap::default()));
    world_grid[2][2] = Some(entities.insert(Tilemap::default()));
    world_grid[2][1] = Some(entities.insert(Tilemap::default()));
    world_grid[2][0] = Some(entities.insert(Tilemap::default()));
    world_grid[1][0] = Some(entities.insert(Tilemap::default()));
    world_grid[6][4] = Some(entities.insert(Tilemap::default()));

    // Place player.
    let handle = world_grid[0][0].unwrap();
    let first_tiles_map = entities.get_mut(&handle);
    first_tiles_map.set(glm::vec2(2, 0), Tile::Player);

    let player = Player::new(AbsolutePosition {
        world: glm::vec2(0, 0),
        tilemap: glm::vec2(2, 0),
        handle,
    });

    (World::new(world_grid), player)
}

#[derive(Debug, Copy, Clone)]
pub struct AbsolutePosition {
    pub world: glm::TVec2<i32>,
    pub tilemap: glm::TVec2<i32>,
    pub handle: Handle<Tilemap>,
}

impl AbsolutePosition {
    pub fn new(
        world: glm::TVec2<i32>,
        tilemap: glm::TVec2<i32>,
        handle: Handle<Tilemap>,
    ) -> Self {
        Self {
            world,
            tilemap,
            handle,
        }
    }
}

impl Add<&glm::TVec2<i32>> for AbsolutePosition {
    type Output = Self;

    fn add(self, delta: &glm::TVec2<i32>) -> Self::Output {
        Self {
            tilemap: self.tilemap + delta,
            world: self.world + delta,
            handle: self.handle,
        }
    }
}

pub struct World {
    pub offset: glm::TVec2<f32>,
    pub grid: [[Option<Handle<Tilemap>>; 5]; 7],
}

impl World {
    pub fn new(grid: [[Option<Handle<Tilemap>>; 5]; 7]) -> Self {
        Self {
            offset: glm::vec2(0., 0.),
            grid,
        }
    }

    pub fn get_sibling_tilemap(
        &self,
        world_pos: &glm::TVec2<i32>,
    ) -> Vec<(Handle<Tilemap>, glm::TVec2<i32>)> {
        let mut siblings = vec![];

        let deltas = [
            glm::vec2(0, 1),
            glm::vec2(0, -1),
            glm::vec2(1, 0),
            glm::vec2(-1, 0),
        ];

        for delta in deltas.iter() {
            let pos = *world_pos + delta;
            if let Some(handle) = self.get_tilemap(&pos) {
                siblings.push((handle, pos));
            };
        }

        siblings
    }

    pub fn get_next_position(
        &self,
        position: &AbsolutePosition,
        delta: &glm::TVec2<i32>,
    ) -> Option<AbsolutePosition> {
        let mut next_position = *position + delta;
        let mut new_tile_position = glm::vec2(0, 0);

        // If next position is not on edges.
        if (0..TILES_COUNT.0).contains(&next_position.tilemap.x)
            && (0..TILES_COUNT.1).contains(&next_position.tilemap.y)
        {
            return Some(AbsolutePosition::new(
                position.world,
                next_position.tilemap,
                position.handle,
            ));
        };

        if !(0..TILEMAPS_COUNT.0).contains(&next_position.world.x)
            || !(0..TILEMAPS_COUNT.1).contains(&next_position.world.y)
        {
            return None;
        }

        if next_position.tilemap.x >= TILES_COUNT.0 {
            new_tile_position = glm::vec2(0, next_position.tilemap.y);
        }

        if next_position.tilemap.x < 0 {
            new_tile_position =
                glm::vec2(TILES_COUNT.0 - 1, next_position.tilemap.y);
        }

        if next_position.tilemap.y >= TILES_COUNT.1 {
            new_tile_position = glm::vec2(next_position.tilemap.x, 0);
        }

        if next_position.tilemap.y < 0 {
            new_tile_position =
                glm::vec2(next_position.tilemap.x, TILES_COUNT.1 - 1);
        }

        // There is a tilemap yay!
        if let Some(handle) = self.get_tilemap(&next_position.world) {
            next_position.tilemap = new_tile_position;
            next_position.handle = handle;
            return Some(next_position);
        }

        None
    }

    pub fn get_tilemap(
        &self,
        world_index: &glm::TVec2<i32>,
    ) -> Option<Handle<Tilemap>> {
        let pos = glm::vec2(world_index.x, world_index.y);

        if !(0..TILEMAPS_COUNT.0).contains(&pos.x)
            || !(0..TILEMAPS_COUNT.1).contains(&pos.y)
        {
            return None;
        };

        self.grid[pos.y as usize][pos.x as usize]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tile {
    Wall(Handle<Wall>),
    Player,
    Ground,
}

#[derive(Debug)]
pub struct Tilemap {
    pub grid: [[Tile; 10]; 20],
    // [
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    // ]
}

/// When we want to access position on the grid, we have to invert x and y.
impl Tilemap {
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.grid[y as usize][x as usize]
    }

    pub fn set(&mut self, position: glm::TVec2<i32>, value: Tile) {
        self.grid[position.y as usize][position.x as usize] = value;
    }

    pub fn find_player(&self) -> Option<glm::TVec2<i32>> {
        let mut player_pos = None;

        for x in 0..TILES_COUNT.0 {
            for y in 0..TILEMAPS_COUNT.1 {
                if Tile::Player == self.get_tile(x, y) {
                    player_pos = Some(glm::vec2(y, x));
                    break;
                }
            }
        }

        player_pos
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        let grid = [
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            [
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
        ];

        Self { grid }
    }
}

// 0 => nothing
// 1 => pushable_wall
// 2 => player
// 3 => wall
// 4 => capacity pusher
// 5 => exit

// grid: [
//     [0, 0, 3, 3, 3, 0, 0, 0, 0, 5],
//     [0, 3, 3, 0, 3, 0, 3, 1, 3, 0],
//     [0, 3, 4, 1, 3, 0, 3, 1, 3, 0],
//     [0, 3, 3, 0, 3, 0, 3, 1, 0, 0],
//     [3, 3, 0, 0, 3, 3, 0, 1, 0, 3],
//     [0, 3, 0, 3, 0, 0, 0, 0, 3, 0],
//     [1, 0, 0, 1, 1, 0, 0, 3, 3, 0],
//     [0, 3, 3, 3, 0, 0, 3, 0, 3, 0],
//     [0, 3, 0, 0, 0, 0, 3, 3, 3, 0],
//     [2, 3, 0, 0, 0, 4, 0, 3, 3, 0],
// ],

// pub fn world_to_grid(
//     tilemap: &Tilemap,
//     world_pos: &glm::TVec2<f32>,
// ) -> glm::TVec2<i32> {
//     let unit_width = tilemap.width / tilemap.size.0 as f32;
//     let unit_height = tilemap.height / tilemap.size.1 as f32;

//     let x = world_pos.x / unit_width;
//     let y = world_pos.y / unit_height;

//     glm::vec2(y as i32, x as i32)
// }

// pub fn grid_to_world(
//     tilemap: &Tilemap,
//     map_pos: &glm::TVec2<i32>,
// ) -> glm::TVec2<f32> {
//     let unit_width = tilemap.width / tilemap.size.0 as f32;
//     let unit_height = tilemap.height / tilemap.size.1 as f32;

//     let x = map_pos.x as f32 * unit_width;
//     let y = map_pos.y as f32 * unit_height;

//     glm::vec2(x, y)
// }
