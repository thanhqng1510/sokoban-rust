use std::fmt::{Display, Formatter};
use std::fmt;


#[derive(Eq, PartialEq)]
pub enum GameplayState {
    Playing,
    Won
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}


#[derive(Default)]
pub struct GameState {
    pub gameplay_state: GameplayState,
    pub moves_count: u32
}

impl GameState {
    pub fn reset(&mut self) {
        self.gameplay_state = GameplayState::Playing;
        self.moves_count = 0;
    }
}
