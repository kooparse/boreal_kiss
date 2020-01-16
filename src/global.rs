use crate::renderer::ShaderManager;
use lazy_static::lazy_static;
use nalgebra_glm as glm;
use std::sync::Mutex;

lazy_static! {
    pub static ref SHADERS: ShaderManager = ShaderManager::build();
    pub static ref PERSPECTIVE_MATRIX: Mutex<glm::Mat4> = unsafe {
        Mutex::new(glm::perspective(
            SCREEN_WIDTH / SCREEN_HEIGHT,
            45.0,
            0.1,
            100.0,
        ))
    };
    pub static ref ORTHO_MATRIX: Mutex<glm::Mat4> = unsafe {
        Mutex::new(glm::ortho(0., SCREEN_WIDTH, 0., SCREEN_HEIGHT, -1., 1.))
    };
    pub static ref VIEW_MATRIX: Mutex<glm::Mat4> = Mutex::new(glm::identity());
}


// Path stuff.
pub static WORLD_FILE_PATH: &str = "assets/maps/world.json";
pub static TILEMAPS_DIR_PATH: &str = "assets/maps/";

// Window stuff.
pub static mut SCREEN_WIDTH: f32 = 1980.;
pub static mut SCREEN_HEIGHT: f32 = 1024.;
pub static mut SCREEN_DPI: u32 = 2;
pub static WITH_VSYNC: bool = false;
pub static MULTISAMPLING: u16 = 8;
pub const GAME_NAME: &str = "Game";

// Map stuff
pub static TILES_COUNT: (i32, i32) = (10, 13);
pub static TILEMAPS_COUNT: (i32, i32) = (5, 7);
pub static TILE_SIZE: f32 = 2.;
pub static TILEMAP_WIDTH: f32 = TILES_COUNT.0 as f32 * TILE_SIZE;
pub static TILEMAP_HEIGHT: f32 = TILES_COUNT.1 as f32 * TILE_SIZE;
