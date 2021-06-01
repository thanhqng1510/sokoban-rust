use specs::World;
use crate::resources::input_queue::InputQueue;
use crate::resources::game_state::GameState;
use crate::resources::var::{BackgroundColor, ComponentTemplateData};

pub mod input_queue;
pub mod game_state;
pub mod var;


pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(GameState::default());
    world.insert(BackgroundColor::default());
    world.insert(ComponentTemplateData::default());
}
