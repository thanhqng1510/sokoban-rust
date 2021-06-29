use specs::{Component, VecStorage, NullStorage};
use std::fmt::{Formatter, Display};
use std::fmt;
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

#[derive(Copy, Clone)]
pub enum BoxSpotColor {
    Beige,
    Black,
    Blue,
    Brown,
    Gray,
    Purple,
    Red,
    Yellow,
    Orange
}

impl Display for BoxSpotColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Beige => write!(f, "beige"),
            Self::Black => write!(f, "black"),
            Self::Blue => write!(f, "blue"),
            Self::Brown => write!(f, "brown"),
            Self::Gray => write!(f, "gray"),
            Self::Purple => write!(f, "purple"),
            Self::Red => write!(f, "red"),
            Self::Yellow => write!(f, "yellow"),
            Self::Orange => write!(f, "orange")
        }
    }
}

#[derive(Copy, Clone)]
pub enum BoxBrightness {
    Dark,
    Bright
}

impl Display for BoxBrightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Dark => write!(f, "dark"),
            Self::Bright => write!(f, "bright")
        }
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

#[derive(Copy, Clone)]
pub enum WallColor {
    Beige,
    Black,
    Brown,
    Gray,
    PompadourPink
}

impl Display for WallColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Beige => write!(f, "beige"),
            Self::Black => write!(f, "black"),
            Self::Brown => write!(f, "brown"),
            Self::Gray => write!(f, "gray"),
            Self::PompadourPink => write!(f, "pompadourpink")
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
pub struct Spot;

impl Spot {
    pub fn new() -> Self {
        Spot {}
    }
}

#[derive(Copy, Clone)]
pub enum FloorMaterial {
    Concrete,
    Dirt,
    Grass,
    Grass2, // Grass but brighter
    Sand
}

impl Display for FloorMaterial {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Concrete => write!(f, "concrete"),
            Self::Dirt => write!(f, "dirt"),
            Self::Grass => write!(f, "grass"),
            Self::Grass2 => write!(f, "grass2"),
            Self::Sand => write!(f, "sand")
        }
    }
}

#[derive(Copy, Clone)]
pub enum FloorType {
    Clean,
    Gravel,
    Plant
}

impl Display for FloorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Clean => write!(f, "clean"),
            Self::Gravel => write!(f, "gravel"),
            Self::Plant => write!(f, "plant")
        }
    }
}
