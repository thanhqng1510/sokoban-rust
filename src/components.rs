use specs::{Component, VecStorage, NullStorage};
use std::collections::HashMap;


#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

impl Position {
    pub fn from(x: u8, y: u8, z: u8) -> Self {
        Position { x, y, z }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub position: Position,
    pub resource_template_path: &'static str,
    pub resource_template_data: HashMap<String, String>
}

impl Renderable {
    pub fn from(position: Position, resource_template_path: &'static str, resource_template_data: HashMap<String, String>) -> Self {
        Renderable { position, resource_template_path, resource_template_data }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

impl Movable {
    pub fn new() -> Self {
        Movable {}
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Blocking;

impl Blocking {
    pub fn new() -> Self {
        Blocking {}
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Directional {
    pub direction: Direction
}

impl Directional {
    pub fn from(direction: Direction) -> Self {
        Directional { direction }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Box;

impl Box {
    pub fn new() -> Self {
        Box {}
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Wall;

impl Wall {
    pub fn new() -> Self {
        Wall {}
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Spot;

impl Spot {
    pub fn new() -> Self {
        Spot {}
    }
}