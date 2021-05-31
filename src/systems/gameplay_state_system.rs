use specs::{System, Write, ReadStorage, Join};
use crate::resources::game_state::{GameState, GameplayState};
use crate::components::{Box, BoxSpot, Renderable};
use std::collections::HashSet;


pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, GameState>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_state, boxes, box_spots, renderables) = data;

        let spot_positions = (&box_spots, &renderables).join()
            .map(|k| (k.1.position.x, k.1.position.y))
            .collect::<HashSet<_>>();

        for (_, renderable) in (&boxes, &renderables).join() {
            if !spot_positions.contains(&(renderable.position.x, renderable.position.y)) {
                game_state.gameplay_state = GameplayState::Playing;
                return;
            }
        }

        game_state.gameplay_state = GameplayState::Won;
    }
}
