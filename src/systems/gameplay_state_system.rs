use specs::{System, ReadStorage, Join, WriteExpect, ReadExpect};
use ggez::audio::SoundSource;
use crate::resources::game_state::{GameState, GameplayState};
use crate::resources::sound_library::SoundLibrary;
use crate::resources::level_data::LevelData;
use crate::components::{Box, Spot, Renderable};
use std::collections::{HashSet, HashMap};


pub struct GameplayStateSystem;

impl GameplayStateSystem {
    pub fn new() -> Self {
        GameplayStateSystem {}
    }
}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        WriteExpect<'a, GameState>,
        WriteExpect<'a, SoundLibrary>,
        ReadExpect<'a, LevelData>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, Spot>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_state,
            mut sound_lib,
            level_data,
            boxes,
            spots,
            renderables) = data;

        if game_state.gameplay_state != GameplayState::Won {
            if let Some(ref mut ingame_music) = sound_lib.music_sound.ingame_music {
                if ingame_music.stopped() { ingame_music.play().unwrap(); }
            }

            if level_data.box_spot_identical_mode == true {
                let spot_positions = (&spots, &renderables).join()
                    .map(|(_, renderable)| ((renderable.position.x, renderable.position.y), &renderable.resource_template_data["spot_color"]))
                    .collect::<HashMap<_, _>>();

                for (_, renderable) in (&boxes, &renderables).join() {
                    if !spot_positions.contains_key(&(renderable.position.x, renderable.position.y)) ||
                        spot_positions[&(renderable.position.x, renderable.position.y)] != &renderable.resource_template_data["box_color"] {
                        game_state.gameplay_state = GameplayState::Playing;
                        return;
                    }
                }
            }
            else {
                let spot_positions = (&spots, &renderables).join()
                    .map(|(_, renderable)| (renderable.position.x, renderable.position.y))
                    .collect::<HashSet<_>>();

                for (_, renderable) in (&boxes, &renderables).join() {
                    if !spot_positions.contains(&(renderable.position.x, renderable.position.y)) {
                        game_state.gameplay_state = GameplayState::Playing;
                        return;
                    }
                }
            }

            game_state.gameplay_state = GameplayState::Won;
            if let Some(ref mut ingame_music) = sound_lib.music_sound.ingame_music {
                if ingame_music.playing() { ingame_music.stop(); }
            }
        }

        if let Some(ref mut victory_music) = sound_lib.music_sound.victory_music {
            if victory_music.stopped() { victory_music.play().unwrap(); }
        }
    }
}
