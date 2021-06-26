use std::{path, fs};

use ggez::{conf, event, GameResult};
use specs::{World, WorldExt};

use crate::game_context::GameContext;
use crate::constant::{AUTHOR, GAME_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, RESOURCE_PREFIX_PATH, SETTING_PATH, GAME_DATA_PATH};
use serde_json::Value;

mod components;
mod entity_builder;
mod systems;
mod game_context;
mod resources;
mod constant;


fn main() -> GameResult {
    let s= fs::read_to_string(SETTING_PATH).unwrap();
    let setting_json: Value = serde_json::from_str(&s).unwrap();

    let s= fs::read_to_string(GAME_DATA_PATH).unwrap();
    let game_data_json: Value = serde_json::from_str(&s).unwrap();

    let context_builder = ggez::ContextBuilder::new(GAME_TITLE, AUTHOR)
        .window_setup(conf::WindowSetup::default()
            .title(GAME_TITLE)
            .vsync(setting_json["vsync"].as_bool().unwrap()))
        .window_mode(conf::WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(path::PathBuf::from(RESOURCE_PREFIX_PATH));
    let (application_context, event_loop) = &mut context_builder.build()?;

    let mut game_context = GameContext::from(World::new());
    game_context.register_components();
    game_context.register_resources();
    game_context.initialize_level(game_data_json["level"].as_u64().unwrap() as u8, application_context);

    event::run(application_context, event_loop, &mut game_context)
}
