use ggez::graphics::Color;
use crate::constant::{DEFAULT_BACKGROUND_COLOR, DEFAULT_GAME_LEVEL};


pub struct LevelData {
    pub background_color: Color,
    pub current_level: u8
}

impl Default for LevelData {
    fn default() -> Self {
        LevelData {
            background_color: Color::from(DEFAULT_BACKGROUND_COLOR),
            current_level: DEFAULT_GAME_LEVEL
        }
    }
}

impl LevelData {
    pub fn reset(&mut self) {
        self.background_color = Color::from(DEFAULT_BACKGROUND_COLOR);
        self.current_level = DEFAULT_GAME_LEVEL;
    }
}
