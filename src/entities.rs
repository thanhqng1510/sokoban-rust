use specs::{World, WorldExt, Builder};
use crate::components::*;


pub const TILE_WIDTH: f32 = 32.;

pub fn create_wall(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/wall_brown.png",
            position: Position { z: 10, ..position }
        })
        .with(Wall {})
        .with(Blocking {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/floor_gravel_grass.png",
            position: Position { z: 5, ..position }
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/box_beige.png",
            position: Position { z: 10, ..position }
        })
        .with(Box {})
        .with(Blocking {})
        .with(Movable {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/spot_beige.png",
            position: Position { z: 9, ..position }
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position, direction: Direction) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/player_down_1.png",
            position: Position { z: 10, ..position }
        })
        .with(Player {})
        .with(Movable {})
        .with(Directional { direction })
        .build();
}
