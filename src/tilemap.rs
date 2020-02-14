use crate::entities::{Entities, Entity, Handle};
use crate::global::{
    TILEMAPS_COUNT, TILEMAPS_DIR_PATH, TILEMAP_HEIGHT, TILEMAP_WIDTH,
    TILES_COUNT, TILE_SIZE, WORLD_FILE_PATH,
};
use crate::player::Player;
use crate::wall::Wall;
use nalgebra_glm as glm;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};
use std::ops::Add;

/// Create the game world.
pub fn init_world_and_player(entities: &mut Entities) -> (World, Player) {
    World::from_file(entities).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub struct AbsolutePosition {
    pub world: glm::TVec2<i32>,
    pub tilemap: glm::TVec2<i32>,
    pub handle: Option<Handle<Tilemap>>,
}

impl AbsolutePosition {
    pub fn new(
        world: glm::TVec2<i32>,
        tilemap: glm::TVec2<i32>,
        handle: Option<Handle<Tilemap>>,
    ) -> Self {
        Self {
            world,
            tilemap,
            handle,
        }
    }

    pub fn to_float_pos(&self) -> glm::TVec3<f32> {
        let mut x = self.world.x as f32 * TILE_SIZE;
        let mut z = self.world.y as f32 * TILE_SIZE;

        x += self.tilemap.x as f32 * TILEMAP_WIDTH;
        z += self.tilemap.y as f32 * TILEMAP_HEIGHT;

        glm::vec3(x, 0., z)
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

type WorldGrid = Vec<Vec<Option<Handle<Tilemap>>>>;
type LocalGrid = Vec<Vec<Tile>>;

pub struct World {
    pub name: String,
    pub offset: glm::TVec2<f32>,
    pub grid: WorldGrid,
}

impl World {
    pub fn new(grid: WorldGrid) -> Self {
        Self {
            name: "uninitialized".to_owned(),
            offset: glm::vec2(0., 0.),
            grid,
        }
    }

    pub fn from_file(
        mut entities: &mut Entities,
    ) -> io::Result<(Self, Player)> {
        let mut world = World::new(vec![vec![None; 5]; 7]);

        let file =
            File::open(WORLD_FILE_PATH).expect("World not found in map files");

        let reader = BufReader::new(file);
        let w: WorldFile = serde_json::from_reader(reader).unwrap();
        world.offset = glm::vec2(w.offset.0, w.offset.1);
        world.name = w.name;

        for row in 0..w.dimension.0 as usize {
            for col in 0..w.dimension.1 as usize {
                if let Some(map_name) = &w.grid[col][row] {
                    let file = File::open(format!(
                        "{}{}.json",
                        TILEMAPS_DIR_PATH, &map_name
                    ))
                    .expect("Tilemap not found in map files");
                    let reader = BufReader::new(file);
                    let map_file: MapFile = serde_json::from_reader(reader)?;

                    let tilemap = Tilemap::from_file(
                        map_file,
                        (row as i32, col as i32),
                        &mut entities,
                    );
                    let handle = entities.insert(tilemap);
                    world.grid[col][row] = Some(handle);
                } else {
                    world.grid[col][row] = None;
                }
            }
        }

        // Place player.
        let player_world_pos = glm::vec2(w.player.0, w.player.1);
        let player_tilemap_pos = glm::vec2(w.player.2, w.player.3);
        let handle = world
            .get_tilemap(&player_world_pos)
            .expect("Error :: No player was set in the world map!");

        let player_tilemap = entities.get_mut(&handle);
        player_tilemap.set(player_tilemap_pos, Tile::Player);
        let player = Player::new(AbsolutePosition {
            world: player_world_pos,
            tilemap: player_tilemap_pos,
            handle: Some(handle),
        });

        Ok((world, player))
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
            glm::vec2(-1, 1),
            glm::vec2(1, 1),
            glm::vec2(1, -1),
            glm::vec2(-1, -1),
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
            next_position.handle = Some(handle);
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
    Void,
}

#[derive(Debug, Deserialize)]
pub struct MapFile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    pathfile: String,
    dimension: (i32, i32),
    grid: [[Option<i32>; 10]; 13],
}

#[derive(Debug, Deserialize)]
pub struct WorldFile {
    name: String,
    offset: (f32, f32),
    player: (i32, i32, i32, i32),
    dimension: (i32, i32),
    grid: Vec<Vec<Option<String>>>,
}

#[derive(Debug)]
pub struct Tilemap {
    pub name: String,
    pub pathfile: String,
    pub grid: LocalGrid,
}

impl From<MapFile> for Tilemap {
    fn from(u: MapFile) -> Self {
        let mut grid = vec![vec![Tile::Ground; 10]; 13];

        for i in 0..u.dimension.0 as usize {
            for j in 0..u.dimension.1 as usize {
                if let Some(val) = &u.grid[j][i] {
                    grid[j][i] = match val {
                        1 => Tile::Ground,
                        _ => Tile::Void,
                    }
                } else {
                    grid[j][i] = Tile::Void;
                }
            }
        }

        Self {
            grid,
            name: u.name,
            pathfile: u.pathfile,
        }
    }
}

/// When we want to access/insert tile on the grid, we have to invert x and y.
impl Tilemap {
    pub fn from_file(
        u: MapFile,
        absolute_pos: (i32, i32),
        entities: &mut Entities,
    ) -> Self {
        let mut grid = vec![vec![Tile::Ground; 10]; 13];

        for i in 0..u.dimension.0 as usize {
            for j in 0..u.dimension.1 as usize {
                if let Some(val) = &u.grid[j][i] {
                    grid[j][i] = match val {
                        1 => Tile::Ground,
                        2 => {
                            let position = AbsolutePosition::new(
                                glm::vec2(absolute_pos.0, absolute_pos.1),
                                glm::vec2(i as i32, j as i32),
                                None,
                            );
                            let wall = Wall::new(position, false);
                            let handle = entities.insert(wall);

                            Tile::Wall(handle)
                        }
                        _ => Tile::Void,
                    }
                } else {
                    grid[j][i] = Tile::Void;
                }
            }
        }

        Self {
            grid,
            name: u.name,
            pathfile: u.pathfile,
        }
    }
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.grid[y as usize][x as usize]
    }

    pub fn set(&mut self, position: glm::TVec2<i32>, value: Tile) {
        self.grid[position.y as usize][position.x as usize] = value;
    }

    #[allow(unused)]
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
