use specs::{World, WorldExt, Builder};
use crate::constant::{WALL_Z, FLOOR_Z, BOX_Z, SPOT_Z, PLAYER_Z};
use crate::components::*;
use std::collections::HashMap;


pub struct EntityBuilder;

impl EntityBuilder {
    pub fn create_wall(world: &mut World, position: Position, wall_shape: &str, wall_color: &str) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: WALL_Z, ..position },
                "/images/wall_{wall_shape}_{wall_color}.png",
                vec![
                    (String::from("wall_shape"), wall_shape.to_string()),
                    (String::from("wall_color"), wall_color.to_string())
                ].into_iter().collect()))
            .with(Wall::new())
            .with(Blocking::new())
            .build();
    }

    pub fn create_floor(world: &mut World, position: Position, floor_type: &str, floor_material: &str) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: FLOOR_Z, ..position },
                "/images/floor_{floor_type}_{floor_material}.png",
                  vec![
                      (String::from("floor_type"), floor_type.to_string()),
                      (String::from("floor_material"), floor_material.to_string())
                  ].into_iter().collect()))
            .build();
    }

    pub fn create_box(world: &mut World, position: Position, box_brightness: &str, box_color: &str) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: BOX_Z, ..position },
                "/images/box_{box_brightness}_{box_color}.png",
                vec![
                      (String::from("box_brightness"), box_brightness.to_string()),
                      (String::from("box_color"), box_color.to_string())
                ].into_iter().collect()))
            .with(Box::new())
            .with(Blocking::new())
            .with(Movable::new())
            .build();
    }

    pub fn create_spot(world: &mut World, position: Position, spot_color: &str) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: SPOT_Z, ..position },
                "/images/spot_{spot_color}.png",
                vec![
                    (String::from("spot_color"), spot_color.to_string())
                ].into_iter().collect()))
            .with(Spot::new())
            .build();
    }

    pub fn create_player(world: &mut World, position: Position, player_direction: Direction) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: PLAYER_Z, ..position },
                "/images/char_down_1.png",
                HashMap::new()))
            .with(Player::new())
            .with(Movable::new())
            .with(Directional::from(player_direction))
            .build();
    }
}
