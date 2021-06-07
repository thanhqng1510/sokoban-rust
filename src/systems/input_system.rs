use specs::{System, ReadStorage, Join, Entities, WriteStorage, WriteExpect};
use crate::components::{Player, Position, Movable, Blocking, Renderable, Directional, Direction};
use crate::resources::input_queue::InputQueue;
use ggez::event::KeyCode;
use crate::constant::{MAP_HEIGHT, MAP_WIDTH};
use std::collections::HashMap;
use crate::resources::game_state::GameState;


type EntityIdMap = HashMap<(u8, u8), u32>;
type IdVec = Vec<u32>;

pub struct InputSystem;

impl InputSystem {
    pub fn new() -> Self {
        InputSystem {}
    }

    fn handle_movement(&self, key: KeyCode, player_pos: &Position, movables: EntityIdMap, blockings: EntityIdMap) -> Option<IdVec> {
        let (checking_range, is_horizontal) = match key {
            KeyCode::Up => ((0..=player_pos.y).rev().collect::<Vec<_>>(), false),
            KeyCode::Down => ((player_pos.y..=MAP_HEIGHT).collect::<Vec<_>>(), false),
            KeyCode::Left => ((0..=player_pos.x).rev().collect::<Vec<_>>(), true),
            KeyCode::Right => ((player_pos.x..=MAP_WIDTH).collect::<Vec<_>>(), true),
            _ => return None
        };

        let mut to_move = Vec::new();

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

        Some(to_move)
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteExpect<'a, InputQueue>,
        WriteExpect<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Directional>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Blocking>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut input_queue,
            mut game_state,
            entities,
            mut renderables,
            mut directionals,
            player,
            movables,
            blockings
        ) = data;

        if let Some(key) = input_queue.pop() {
            for(player_direction, _player) in (&mut directionals, &player).join() {
                player_direction.direction = match key {
                    KeyCode::Up => Direction::Up,
                    KeyCode::Down => Direction::Down,
                    KeyCode::Left => Direction::Left,
                    KeyCode::Right => Direction::Right,
                    _ => player_direction.direction
                }
            }

            let mut to_move = None;
            for (renderable_player, _player) in (&renderables, &player).join() {
                let movables = (&entities, &movables, &renderables)
                    .join()
                    .map(|t| ((t.2.position.x, t.2.position.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let blockings = (&entities, &blockings, &renderables)
                    .join()
                    .map(|t| ((t.2.position.x, t.2.position.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                to_move = self.handle_movement(key, &renderable_player.position, movables, blockings);
            }

            if let Some(to_move) = to_move {
                if to_move.len() > 0 { game_state.moves_count += 1; }

                for id in to_move {
                    let renderable = renderables.get_mut(entities.entity(id)).unwrap();
                    match key {
                        KeyCode::Up => renderable.position.y -= 1,
                        KeyCode::Down => renderable.position.y += 1,
                        KeyCode::Left => renderable.position.x -= 1,
                        KeyCode::Right => renderable.position.x += 1,
                        _ => ()
                    }
                }
            }
        }
    }
}
