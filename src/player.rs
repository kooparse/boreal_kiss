use crate::entities::{Entities, Entity, Handle};
use crate::input::{Input, Key};
use crate::map::{AbsolutePosition, Tile, Tilemap, World};
use crate::time::Time;
use crate::wall::Wall;
use nalgebra_glm as glm;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}

impl MoveDirection {
    fn to_grid_delta(&self) -> glm::TVec2<i32> {
        match self {
            Self::Up => glm::vec2(0, 1),
            Self::Down => glm::vec2(0, -1),
            Self::Right => glm::vec2(-1, 0),
            Self::Left => glm::vec2(1, 0),
        }
    }

    fn to_world_delta(&self) -> glm::TVec3<f32> {
        match self {
            Self::Up => glm::vec3(0., 1., 1.),
            Self::Down => glm::vec3(0., 1., -1.),
            Self::Right => glm::vec3(-1., 1., 0.),
            Self::Left => glm::vec3(1., 1., 0.),
        }
    }

    // TODO: Not precise 'cause of tick delta.
    fn is_near(
        &self,
        world_pos: &glm::TVec2<f32>,
        end_pos: &glm::TVec2<f32>,
    ) -> bool {
        return match self {
            Self::Up => world_pos >= end_pos,
            Self::Down => world_pos <= end_pos,
            Self::Right => world_pos <= end_pos,
            Self::Left => world_pos >= end_pos,
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MoveAnimation {
    direction: MoveDirection,
    destination: glm::TVec2<f32>,
}

pub struct Player {
    // On where map the player is.
    // pub map_handle: Handle<Tilemap>,
    pub tilemap_pos: AbsolutePosition,
    pub world_pos: glm::TVec3<f32>,
    // Animation...
    // move_animation: Option<MoveAnimation>,
}

impl Player {
    pub fn new(tilemap_pos: AbsolutePosition) -> Self {
        let world_pos = glm::vec3(
            tilemap_pos.tilemap.x as f32,
            1.,
            tilemap_pos.tilemap.y as f32,
        );

        Self {
            // map_handle,
            tilemap_pos,
            world_pos,
            // move_animation: None,
        }
    }

    pub fn move_player(
        &mut self,
        time: &Time,
        input: &mut Input,
        mut world: &mut World,
        mut entities: &mut Entities,
    ) {
        let mut direction: Option<MoveDirection> = None;
        let pressed_duration = Duration::from_millis(50);

        if input.is_pressed_delay(pressed_duration, Key::W) {
            direction = Some(MoveDirection::Up);
        };

        if input.is_pressed_delay(pressed_duration, Key::S) {
            direction = Some(MoveDirection::Down);
        };

        if input.is_pressed_delay(pressed_duration, Key::D) {
            direction = Some(MoveDirection::Right);
        };

        if input.is_pressed_delay(pressed_duration, Key::A) {
            direction = Some(MoveDirection::Left);
        };

        let tilemap = entities.maps.get(&self.tilemap_pos.handle);

        // let mut walls: Vec<Handle<Wall>> = vec![];

        // entities.walls.iter().for_each(|(w, h)| {
        //     if w.move_animation.is_some() {
        //         walls.push(*h);
        //     }
        // });

        // for h in walls {
        //     let wall = entities.walls.get_mut(&h);

        //     if let Some(anim) = &wall.move_animation {
        //         if anim.direction.is_near(&wall.world_pos, &anim.destination) {
        //             let delta = anim.direction.to_grid_delta();
        //             let map_pos = wall.map_pos + delta;
        //             // map.set(wall.map_pos, Tile::Ground);
        //             map.set(map_pos, Tile::Wall(h));
        //             wall.map_pos = map_pos;

        //             wall.move_animation = None;
        //         } else {
        //             let anim_delta = anim.direction.to_world_delta();
        //             wall.world_pos += anim_delta * 10. * (time.dt as f32);
        //         }
        //     }
        // }

        // if let Some(anim) = &self.move_animation {
        //     if anim.direction.is_near(&self.world_pos, &anim.destination) {
        //         let delta = anim.direction.to_grid_delta();
        //         // let player_pos = map.find_player().unwrap();
        //         let map_pos = self.map_pos + delta;

        //         map.set(self.map_pos, Tile::Ground);
        //         map.set(map_pos, Tile::Player);
        //         self.map_pos = map_pos;

        //         self.move_animation = None;
        //     } else {
        //         let anim_delta = anim.direction.to_world_delta();
        //         self.world_pos += anim_delta * 10. * (time.dt as f32);
        //     }

        //     return;
        // }

        if input.is_pressed_once(Key::Space) {
            dbg!(&self.tilemap_pos, tilemap.find_player());
        }

        if input.modifiers.shift {
            return;
        }

        // Only if input is pressed.
        if let Some(dir) = direction {
            let delta = dir.to_grid_delta();

            if let Some(next_pos) = self.can_move(
                &mut world,
                &self.tilemap_pos.clone(),
                delta,
                &entities,
            ) {
                let tilemap = entities.maps.get_mut(&self.tilemap_pos.handle);
                tilemap.set(self.tilemap_pos.tilemap, Tile::Ground);
                let tilemap = entities.maps.get_mut(&next_pos.handle);
                tilemap.set(next_pos.tilemap, Tile::Player);
                self.tilemap_pos = next_pos;

                self.world_pos = self.world_pos + dir.to_world_delta();

                // self.move_animation = Some(MoveAnimation {
                //     direction: dir,
                //     destination: self.world_pos + dir.to_world_delta() * 2.,
                // });

                return;
            }

            // let mut pushable_walls: Vec<glm::TVec2<i32>> = vec![];
            // let is_pushable = self.is_wall_pushable(
            //     &mut map,
            //     // current pos.
            //     self.map_pos,
            //     delta,
            //     &mut pushable_walls,
            // );

            // if !is_pushable {
            //     return;
            // }

            // for (_, pos) in pushable_walls.iter().enumerate() {
            //     // pushable_walls.iter().enumerate().for_each(|(_, pos)| {
            //     self.move_animation = Some(MoveAnimation {
            //         direction: dir,
            //         destination: self.world_pos + dir.to_world_delta() * 2.,
            //     });

            //     if let Tile::Wall(handle) =
            //         map.grid[pos.x as usize][pos.y as usize]
            //     {
            //         let wall = entities.walls.get_mut(&handle);

            //         wall.move_animation = Some(MoveAnimation {
            //             direction: dir,
            //             destination: wall.world_pos + dir.to_world_delta() * 2.,
            //         });
            //     }
            // };
        }
    }

    // pub fn is_wall_pushable(
    //     &self,
    //     mut map: &mut Tilemap,
    //     wall_pos: glm::TVec2<i32>,
    //     delta: glm::TVec2<i32>,
    //     mut pushable_position: &mut Vec<glm::TVec2<i32>>,
    // ) -> bool {
    //     let projection = wall_pos + delta;

    //     if !(0..map.size.0).contains(&(projection.x as usize))
    //         || !(0..map.size.1).contains(&(projection.y as usize))
    //     {
    //         return false;
    //     }

    //     let value = &map.grid[projection.x as usize][projection.y as usize];

    //     if pushable_position.len() > 1 {
    //         return false;
    //     }

    //     match value {
    //         Tile::Wall(_) => {
    //             pushable_position.push(projection);
    //             return self.is_wall_pushable(
    //                 &mut map,
    //                 projection,
    //                 delta,
    //                 &mut pushable_position,
    //             );
    //         }
    //         Tile::Ground => {
    //             return true;
    //         }
    //         _ => {
    //             return false;
    //         }
    //     }
    // }

    pub fn can_move(
        &mut self,
        world: &mut World,
        pos: &AbsolutePosition,
        delta: glm::TVec2<i32>,
        entities: &Entities,
    ) -> Option<AbsolutePosition> {
        if let Some(new_position) = world.get_next_position(pos, &delta) {
            let tilemap = entities.get(&new_position.handle);
            let tile = tilemap
                .get_tile(new_position.tilemap.x, new_position.tilemap.y);
            // Can move...
            if Tile::Ground == tile {
                return Some(new_position);
            }
        }

        return None;
    }
}
