use ggez::graphics::Color;


pub struct GameVars {
    pub background_color: Color
}

impl Default for GameVars {
    fn default() -> Self {
        GameVars { background_color: Color::from_rgba(180, 180, 180, 255) }
    }
}
