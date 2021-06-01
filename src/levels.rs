use std::fs;
use specs::World;
use crate::entities::{create_player, create_wall, create_box, create_floor, create_spot};
use crate::components::{Position, Direction, WallColor, WallShape};
use std::cmp::min;
use crate::constant::{MAX_LEVEL, RESOURCE_PREFIX_PATH};


pub fn load_map(world: &mut World, map_string: String) {
    let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

    for (y, &row) in rows.iter().enumerate() {
        let columns = row.split(' ').collect::<Vec<_>>();

        for (x, &column) in columns.iter().enumerate() {
            let position = Position { x: x as u8, y: y as u8, z: 0 };

            match column {
                "." => create_floor(world, position),
                "W" => {
                    create_wall(world, position, WallColor::Beige, WallShape::Round);
                    create_floor(world, position);
                },
                "P" => {
                    create_player(world, position, Direction::Down);
                    create_floor(world, position);
                },
                "B" => {
                    create_box(world, position);
                    create_floor(world, position);
                },
                "S" => {
                    create_spot(world, position);
                    create_floor(world, position);
                },
                "N" => (),
                c => panic!("Unrecognized map item {}", c)
            }
        }
    }
}

pub fn initialize_level(world: &mut World, level: u8) {
    let level = min(level, MAX_LEVEL);
    let map= &fs::read_to_string(format!("{}/maps/map_{}.txt", RESOURCE_PREFIX_PATH, level))
        .expect(&format!("Unable to read file. Check if level {} exists!", level));

    load_map(world, map.to_string());
}
