use crate::input::{Input, Key};
use crate::map::Map;

pub struct Player {
    capacity: usize,
    pub map_pos: (usize, usize),
}

impl Default for Player {
    fn default() -> Self {
        Self {
            capacity: 1,
            map_pos: (0, 0),
        }
    }
}

impl Player {
    pub fn move_on_grid(&mut self, input: &mut Input, mut map: &mut Map) {
        let mut move_delta = (0, 0);

        if input.is_pressed_once(Key::W) {
            move_delta.1 += 1;
        };

        if input.is_pressed_once(Key::S) {
            move_delta.1 -= 1;
        };

        if input.is_pressed_once(Key::D) {
            move_delta.0 -= 1;
        };

        if input.is_pressed_once(Key::A) {
            move_delta.0 += 1;
        };

        // If not moved, we don't do nothing.
        if move_delta.0 == 0 && move_delta.1 == 0 {
            map.grid[self.map_pos.0][self.map_pos.1] = 2;
            return;
        }

        let projection =
            (self.map_pos.0 + move_delta.0, self.map_pos.1 + move_delta.1);

        if self.can_move(map, self.map_pos, move_delta) {
            if map.grid[projection.0][projection.1] == 4 {
                self.capacity += 1;
            }

            // Move player if next position is free.
            map.grid[self.map_pos.0][self.map_pos.1] = 0;
            self.map_pos = projection;
            map.grid[projection.0][projection.1] = 2;
            return;
        };

        let mut pushable_walls: Vec<(usize, usize)> = vec![];
        let is_pushable = self.is_wall_pushable(
            &mut map,
            // current pos.
            (self.map_pos.0, self.map_pos.1),
            move_delta,
            &mut pushable_walls,
        );

        if !is_pushable {
            return;
        }

        pushable_walls.iter().enumerate().for_each(|(index, pos)| {
            if index == 0 {
                map.grid[pos.0][pos.1] = 0;
                map.grid[self.map_pos.0][self.map_pos.1] = 0;
                self.map_pos = projection;
                map.grid[projection.0][projection.1] = 2;
            }

            map.grid[pos.0][pos.1] = 1;
        });
    }

    pub fn is_wall_pushable(
        &self,
        mut map: &mut Map,
        wall_pos: (usize, usize),
        delta: (usize, usize),
        mut pushable_position: &mut Vec<(usize, usize)>,
    ) -> bool {
        let projection = (wall_pos.0 + delta.0, wall_pos.1 + delta.1);

        if !(0..map.size.0).contains(&projection.0)
            || !(0..map.size.1).contains(&projection.1)
        {
            return false;
        }

        let value = map.grid[projection.0][projection.1];

        if value == 1 && pushable_position.len() <= self.capacity - 1 {
            pushable_position.push(projection);
            return self.is_wall_pushable(
                &mut map,
                projection,
                delta,
                &mut pushable_position,
            );
        } else if value == 0 {
            pushable_position.push(projection);
            return true;
        } else {
            return false;
        }
    }

    pub fn can_move(
        &self,
        map: &mut Map,
        pos: (usize, usize),
        delta: (usize, usize),
    ) -> bool {
        let projection = (pos.0 + delta.0, pos.1 + delta.1);

        if !(0..map.size.0).contains(&projection.0)
            || !(0..map.size.1).contains(&projection.1)
        {
            return false;
        };

        return map.grid[projection.0][projection.1] == 0
            || map.grid[projection.0][projection.1] == 4;
    }
}
