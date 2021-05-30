use specs::World;
use crate::resources::input_queue::InputQueue;

pub mod input_queue;


pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}
