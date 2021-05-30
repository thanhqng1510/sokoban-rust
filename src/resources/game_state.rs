pub enum GameplayState {
    Playing,
    Won
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}


#[derive(Default)]
pub struct GameState {
    pub gameplay_state: GameplayState,
    pub moves_count: u32
}
