use std::path;

use ggez::{conf, event, GameResult};
use specs::{World, WorldExt};

use crate::game_context::GameContext;
use crate::constant::{AUTHOR, GAME_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, RESOURCE_PREFIX_PATH};

mod components;
mod entity_builder;
mod systems;
mod game_context;
mod resources;
mod constant;


fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new(GAME_TITLE, AUTHOR)
        .window_setup(conf::WindowSetup::default()
            .title(GAME_TITLE)
            .vsync(false))
        .window_mode(conf::WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(path::PathBuf::from(RESOURCE_PREFIX_PATH));
    let (application_context, event_loop) = &mut context_builder.build()?;

    let mut game_context = GameContext::from(World::new(), application_context);
    game_context.play_ingame_music();
    game_context.register_components();
    game_context.register_resources();
    game_context.initialize_level(0);

    event::run(application_context, event_loop, &mut game_context)
}
