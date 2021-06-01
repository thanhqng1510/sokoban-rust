use specs::{World, WorldExt, Builder};
use crate::constant::{WALL_Z, FLOOR_Z, BOX_Z, SPOT_Z, PLAYER_Z};
use crate::components::*;


pub fn create_wall(world: &mut World, position: Position, color: WallColor, shape: WallShape) {
    world.create_entity()
        .with(Renderable {
            position: Position { z: WALL_Z, ..position },
            resource_template_path: "/images/wall_{shape}_{color}.png",
            template_data: Some(vec![
                ("color".to_string(), color.to_string()),
                ("shape".to_string(), shape.to_string())
            ].into_iter().collect())
        })
        .with(Wall {})
        .with(Blocking {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            position: Position { z: FLOOR_Z, ..position },
            resource_template_path: "/images/floor_gravel_grass.png",
            template_data: None
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            position: Position { z: BOX_Z, ..position },
            resource_template_path: "/images/box_beige.png",
            template_data: None
        })
        .with(Box {})
        .with(Blocking {})
        .with(Movable {})
        .build();
}

pub fn create_spot(world: &mut World, position: Position) {
    world.create_entity()
        .with(Renderable {
            position: Position { z: SPOT_Z, ..position },
            resource_template_path: "/images/spot_beige.png",
            template_data: None
        })
        .with(Spot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position, direction: Direction) {
    world.create_entity()
        .with(Renderable {
            position: Position { z: PLAYER_Z, ..position },
            resource_template_path: "/images/char_down_1.png",
            template_data: None
        })
        .with(Player {})
        .with(Movable {})
        .with(Directional { direction })
        .build();
}
