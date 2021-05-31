use std::path;

use ggez::{conf, event, GameResult};
use specs::{World, WorldExt};

use crate::components::register_components;
use crate::game::Game;
use crate::levels::initialize_level;
use crate::resources::register_resources;

mod components;
mod entities;
mod levels;
mod systems;
mod game;
mod resources;


fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Rust Sokoban", "Thanh Nguyen - Ly Nguyen")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban").vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(800., 600.))
        .add_resource_path(path::PathBuf::from("./resources"));
    let (context, event_loop) = &mut context_builder.build()?;

    let game = &mut Game { world: World::new() };
    register_components(&mut game.world);
    register_resources(&mut game.world);
    initialize_level(&mut game.world, 0);

    event::run(context, event_loop, game)
}
