use serde_json::json;
use lazy_static::lazy_static;


pub const GAME_TITLE: &'static str = "Rust Sokoban";
pub const AUTHOR: &'static str = "Thanh Nguyen - Ly Nguyen";

pub const WINDOW_WIDTH: f32 = TILE_SIZE * MAP_WIDTH as f32;
pub const WINDOW_HEIGHT: f32 = TILE_SIZE * MAP_HEIGHT as f32;

pub const RESOURCE_PREFIX_PATH: &'static str = "./resources";

lazy_static! {
    pub static ref RESOURCE_PATH: serde_json::Value = json!({
        "BOX_BEIGE": "/images/box_beige.png",
        "": ""
    });
}

pub const MAP_WIDTH: u8 = 10;
pub const MAP_HEIGHT: u8 = 8;

pub const TILE_SIZE: f32 = 64.;

pub const MAX_LEVEL: u8 = 4;

pub const WALL_Z: u8 = 10;
pub const FLOOR_Z: u8 = 5;
pub const BOX_Z: u8 = 10;
pub const SPOT_Z: u8 = 9;
pub const PLAYER_Z: u8 = 10;

lazy_static! {
    pub static ref BACKGROUND_COLOR: serde_json::Value = json!({
        "r": 0.7,
        "g": 0.7,
        "b": 0.7,
        "a": 1.
    });
}
