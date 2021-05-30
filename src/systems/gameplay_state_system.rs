use specs::{System, Write, ReadStorage, Join};
use crate::resources::game_state::{GameState, GameplayState};
use crate::components::{Box, BoxSpot, Position};
use std::collections::HashSet;


pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, GameState>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_state, boxes, box_spots, positions) = data;

        let spot_positions = (&box_spots, &positions).join()
            .map(|k| (k.1.x, k.1.y))
            .collect::<HashSet<_>>();

        for (_, position) in (&boxes, &positions).join() {
            if !spot_positions.contains(&(position.x, position.y)) {
                game_state.gameplay_state = GameplayState::Playing;
                return;
            }
        }

        game_state.gameplay_state = GameplayState::Won;
    }
}
