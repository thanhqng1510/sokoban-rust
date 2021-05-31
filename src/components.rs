use specs::{Component, VecStorage, NullStorage, World, WorldExt};


#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub resource_path: &'static str,
    pub position: Position
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
pub struct BoxSpot;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Blocking;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Directional {
    pub direction: Direction
}

pub fn register_components(world: &mut World) {
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Blocking>();
    world.register::<Directional>();
}
