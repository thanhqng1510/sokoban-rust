use specs::{Component, VecStorage, NullStorage, World, WorldExt};
use std::collections::HashMap;


#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

#[derive(Copy, Clone)]
pub enum WallColor {
    Beige,
    Black,
    Brown,
    Gray
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub position: Position,
    pub resource_template_path: &'static str,
    pub template_data: HashMap<&'static str, &'static str>
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Wall;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Box;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Spot;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Blocking;

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

pub fn register_components(world: &mut World) {
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<Spot>();
    world.register::<Movable>();
    world.register::<Blocking>();
    world.register::<Directional>();
}
