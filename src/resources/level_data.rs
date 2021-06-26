use ggez::graphics::Color;
use crate::constant::{DEFAULT_BACKGROUND_COLOR, DEFAULT_BOX_SPOT_IDENTICAL_MODE, DEFAULT_GAME_LEVEL};


pub struct LevelData {
    pub background_color: Color,
    pub box_spot_identical_mode: bool,
    pub current_level: u8
}

impl Default for LevelData {
    fn default() -> Self {
        LevelData {
            background_color: Color::from(DEFAULT_BACKGROUND_COLOR),
            box_spot_identical_mode: DEFAULT_BOX_SPOT_IDENTICAL_MODE,
            current_level: DEFAULT_GAME_LEVEL
        }
    }
}

impl LevelData {
    pub fn reset(&mut self) {
        self.background_color = Color::from(DEFAULT_BACKGROUND_COLOR);
        self.box_spot_identical_mode = DEFAULT_BOX_SPOT_IDENTICAL_MODE;
        self.current_level = DEFAULT_GAME_LEVEL;
    }
}
