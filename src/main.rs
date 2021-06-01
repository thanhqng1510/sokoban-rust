use std::path;

use ggez::{conf, event, GameResult};
use specs::{World, WorldExt};

use crate::components::register_components;
use crate::game::Game;
use crate::levels::initialize_level;
use crate::resources::register_resources;
use crate::constant::{AUTHOR, GAME_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, RESOURCE_PREFIX_PATH};

mod components;
mod entities;
mod levels;
mod systems;
mod game;
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
    let (context, event_loop) = &mut context_builder.build()?;

    let game = &mut Game { world: World::new() };
    register_components(&mut game.world);
    register_resources(&mut game.world);
    initialize_level(&mut game.world, 0);

    event::run(context, event_loop, game)
}
