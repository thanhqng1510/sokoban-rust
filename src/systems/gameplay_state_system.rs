use specs::{System, Write, ReadStorage, Join};
use ggez::audio::SoundSource;
use crate::resources::game_state::{GameState, GameplayState};
use crate::resources::sound_library::SoundLibrary;
use crate::components::{Box, Spot, Renderable};
use std::collections::HashSet;


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
        ReadStorage<'a, Box>,
        ReadStorage<'a, Spot>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_state,
            mut sound_lib,
            boxes,
            box_spots,
            renderables) = data;

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

        if let Some(ref mut ingame_music) = sound_lib.music_sound.ingame_music { ingame_music.stop(); }
        if let Some(ref mut victory_music) = sound_lib.music_sound.victory_music {
            if !victory_music.playing() { victory_music.play().unwrap(); }
        }
    }
}
