use specs::{Component, VecStorage, NullStorage, World, WorldExt};
use std::fmt::{Formatter, Display};
use std::fmt;


#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub position: Position,
    pub resource_template_path: &'static str,
}

#[derive(Copy, Clone)]
pub enum WallColor {
    Beige,
    Black,
    Brown,
    Gray
}

impl Display for WallColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Beige => write!(f, "beige"),
            Self::Black => write!(f, "black"),
            Self::Brown => write!(f, "brown"),
            Self::Gray => write!(f, "gray")
        }
    }
}

#[derive(Copy, Clone)]
pub enum WallShape {
    Square,
    Round
}

impl Display for WallShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Square => write!(f, "square"),
            Self::Round => write!(f, "round")
        }
    }
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

#[derive(Copy, Clone)]
pub enum FloorMaterial {
    Concrete,
    Dirt,
    Grass,
    Sand
}

impl Display for FloorMaterial {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Concrete => write!(f, "concrete"),
            Self::Dirt => write!(f, "dirt"),
            Self::Grass => write!(f, "grass"),
            Self::Sand => write!(f, "sand")
        }
    }
}

#[derive(Copy, Clone)]
pub enum FloorType {
    Clean,
    Gravel
}

impl Display for FloorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Clean => write!(f, "clean"),
            Self::Gravel => write!(f, "gravel")
        }
    }
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
