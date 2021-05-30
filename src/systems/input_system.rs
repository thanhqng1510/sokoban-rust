use specs::{System, ReadStorage, Write, Join, Entities, WriteStorage};
use crate::components::{Player, Position, Movable, Blocking};
use crate::resources::input_queue::InputQueue;
use ggez::event::KeyCode;
use crate::levels::{MAP_HEIGHT, MAP_WIDTH};
use std::collections::HashMap;


type EntityIdMap = HashMap<(u8, u8), u32>;
type IdVec = Vec<u32>;

pub struct InputSystem {}

impl InputSystem {
    fn handle_movement(&self, key: KeyCode, player_pos: &Position, movables: EntityIdMap, blockings: EntityIdMap, to_move: &mut IdVec) {
        let (checking_range, is_horizontal) = match key {
            KeyCode::Up => ((0..=player_pos.y).rev().collect::<Vec<_>>(), false),
            KeyCode::Down => ((player_pos.y..=MAP_HEIGHT).collect::<Vec<_>>(), false),
            KeyCode::Left => ((0..=player_pos.x).rev().collect::<Vec<_>>(), true),
            KeyCode::Right => ((player_pos.x..=MAP_WIDTH).collect::<Vec<_>>(), true),
            _ => return
        };

        for idx in checking_range {
            let pos = if is_horizontal { (idx, player_pos.y) } else { (player_pos.x, idx) };

            match movables.get(&pos) {
                Some(id) => to_move.push(*id),
                None => {
                    if let Some(_) = blockings.get(&pos) { to_move.clear(); }
                    break;
                }
            }
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Blocking>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut input_queue,
            entities,
            mut positions,
            player,
            movables,
            blockings
        ) = data;

        let mut to_move = Vec::new();

        if let Some(key) = input_queue.pop() {
            for (player_pos, _player) in (&positions, &player).join() {
                let movables = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let blockings = (&entities, &blockings, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                self.handle_movement(key, player_pos, movables, blockings, &mut to_move);
            }

            for id in to_move {
                let position = positions.get_mut(entities.entity(id)).unwrap();
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => ()
                }
            }
        }
    }
}
