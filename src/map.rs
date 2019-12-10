pub struct Map {
    pub size: (usize, usize),
    pub grid: [[i32; 10]; 10],
}

impl Default for Map {
    fn default() -> Self {
        Self {
            size: (10, 10),
            grid: [
                [0, 0, 3, 3, 3, 0, 0, 0, 0, 5],
                [0, 3, 3, 0, 3, 0, 3, 1, 3, 0],
                [0, 3, 4, 1, 3, 0, 3, 1, 3, 0],
                [0, 3, 3, 0, 3, 0, 3, 1, 0, 0],
                [3, 3, 0, 0, 3, 3, 0, 1, 0, 3],
                [0, 3, 0, 3, 0, 0, 0, 0, 3, 0],
                [1, 0, 0, 1, 1, 0, 0, 3, 3, 0],
                [0, 3, 3, 3, 0, 0, 3, 0, 3, 0],
                [0, 3, 0, 0, 0, 0, 3, 3, 3, 0],
                [2, 3, 0, 0, 0, 4, 0, 3, 3, 0],
            ],
        }
    }
}

// 0 => nothing
// 1 => pushable_wall
// 2 => player
// 3 => wall
// 4 => capacity pusher
// 5 => exit
