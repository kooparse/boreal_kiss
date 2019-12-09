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

pub static mut SCREEN_WIDTH: f32 = 1600.;
pub static mut SCREEN_HEIGHT: f32 = 1000.;
pub static mut SCREEN_DPI: u32 = 2;
pub static WITH_VSYNC: bool = false;
pub static MULTISAMPLING: u16 = 8;

pub const GAME_NAME: &str = "Game";
