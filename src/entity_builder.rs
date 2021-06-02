use specs::{World, WorldExt, Builder};
use crate::constant::{WALL_Z, FLOOR_Z, BOX_Z, SPOT_Z, PLAYER_Z};
use crate::components::*;


pub struct EntityBuilder;

impl EntityBuilder {
    pub fn create_wall(world: &mut World, position: Position) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: WALL_Z, ..position },
                "/images/wall_{wall_shape}_{wall_color}.png"))
            .with(Wall::new())
            .with(Blocking::new())
            .build();
    }

    pub fn create_floor(world: &mut World, position: Position) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: FLOOR_Z, ..position },
                "/images/floor_{floor_type}_{floor_material}.png"))
            .build();
    }

    pub fn create_box(world: &mut World, position: Position) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: BOX_Z, ..position },
                "/images/box_bright_beige.png"))
            .with(Box::new())
            .with(Blocking::new())
            .with(Movable::new())
            .build();
    }

    pub fn create_spot(world: &mut World, position: Position) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: SPOT_Z, ..position },
                "/images/spot_beige.png"))
            .with(Spot::new())
            .build();
    }

    pub fn create_player(world: &mut World, position: Position, direction: Direction) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: PLAYER_Z, ..position },
                "/images/char_down_1.png"))
            .with(Player::new())
            .with(Movable::new())
            .with(Directional::from(direction))
            .build();
    }
}
