use serde::Deserialize;

/// File describing the world with all
/// tilemaps.
#[derive(Debug, Deserialize)]
pub struct WorldFile {
    // Name of the world map, maybe there will be 
    // multiple world maps later.
    name: String,
    // If we don't want to start at (0., 0.), we could change it here.
    offset: (f32, f32),
    // Position of the player on the worldmap but also on the current tilemap.
    // So (player.0, player.1) is the position on the worldmap and
    // (player.2, player.3) is for the tilemap.
    player: (i32, i32, i32, i32),
    // Dimension of the worldmap.
    dimension: (i32, i32),
    // The actual grid, with the optional name of tilemap files.
    grid: Vec<Vec<Option<String>>>,
}

impl WorldFile {
    /// Save our current world to the file system.
    pub fn save() {

    }

    /// Load the world from the file system, to the main memory.
    pub fn load() {

    }
}





