use specs::{World, WorldExt, Builder};
use crate::constant::{WALL_Z, FLOOR_Z, BOX_Z, SPOT_Z, PLAYER_Z};
use crate::components::*;


pub fn create_wall(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/wall_brown.png",
            position: Position { z: WALL_Z, ..position }
        })
        .with(Wall {})
        .with(Blocking {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/floor_gravel_grass.png",
            position: Position { z: FLOOR_Z, ..position }
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/box_beige.png",
            position: Position { z: BOX_Z, ..position }
        })
        .with(Box {})
        .with(Blocking {})
        .with(Movable {})
        .build();
}

pub fn create_spot(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/spot_beige.png",
            position: Position { z: SPOT_Z, ..position }
        })
        .with(Spot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position, direction: Direction) {
    world.create_entity()
        .with(Renderable {
            resource_path: "/images/player_down_1.png",
            position: Position { z: PLAYER_Z, ..position }
        })
        .with(Player {})
        .with(Movable {})
        .with(Directional { direction })
        .build();
}
