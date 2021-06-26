pub const GAME_TITLE: &'static str = "Rust Sokoban";
pub const AUTHOR: &'static str = "Thanh Nguyen - Ly Nguyen";

pub const WINDOW_WIDTH: f32 = TILE_SIZE * MAP_WIDTH as f32;
pub const WINDOW_HEIGHT: f32 = TILE_SIZE * MAP_HEIGHT as f32;

pub const RESOURCE_PREFIX_PATH: &'static str = "./resources";
pub const SETTING_PATH: &'static str = "./data/settings.json";
pub const GAME_DATA_PATH: &'static str = "./data/game_data.json";

pub const MAP_WIDTH: u8 = 12;
pub const MAP_HEIGHT: u8 = 9;

pub const TILE_SIZE: f32 = 64.;

pub const MAX_LEVEL: u8 = 4;

pub const WALL_Z: u8 = 10;
pub const FLOOR_Z: u8 = 5;
pub const BOX_Z: u8 = 10;
pub const SPOT_Z: u8 = 9;
pub const PLAYER_Z: u8 = 10;

pub const SUPPORTED_SOUND_FILE_EXT: [&'static str; 4] = ["wav", "mp3", "ogg", "flac"];

pub const DEFAULT_BACKGROUND_COLOR: (u8, u8, u8, u8) = (180, 180, 180, 255);
pub const DEFAULT_BOX_SPOT_IDENTICAL_MODE: bool = false;
pub const DEFAULT_GAME_LEVEL: u8 = 0;
