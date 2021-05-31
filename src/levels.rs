use std::fs;
use specs::World;
use crate::entities::{create_player, create_wall, create_box, create_floor, create_box_spot};
use crate::components::Position;


pub const MAP_WIDTH: u8 = 8;
pub const MAP_HEIGHT: u8 = 9;

pub fn load_map(world: &mut World, map_string: String) {
    let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

    for (y, &row) in rows.iter().enumerate() {
        let columns = row.split(' ').collect::<Vec<_>>();

        for (x, &column) in columns.iter().enumerate() {
            let position = Position { x: x as u8, y: y as u8, z: 0 };

            match column {
                "." => create_floor(world, position),
                "W" => {
                    create_wall(world, position);
                    create_floor(world, position);
                },
                "P" => {
                    create_player(world, position);
                    create_floor(world, position);
                },
                "B" => {
                    create_box(world, position);
                    create_floor(world, position);
                },
                "S" => {
                    create_box_spot(world, position);
                    create_floor(world, position);
                },
                "N" => (),
                c => panic!("Unrecognized map item {}", c)
            }
        }
    }
}

pub fn initialize_level(world: &mut World, level: i8) {
    /*const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";*/

    let map= &*fs::read_to_string(format!("D:/Music/FPC/Code/Rust/sukoban-rust/resources/maps/map_{}.txt", level))
        .expect(&*format!("Unable to read file. Check if level {} exists?", level));
    load_map(world, map.to_string());
}
