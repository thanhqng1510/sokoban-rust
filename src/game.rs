use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use ggez::audio::{SoundData, Source};
use ggez::audio::SoundSource;
use crate::systems::rendering_system::RenderingSystem;
use crate::resources::input_queue::InputQueue;
use crate::systems::input_system::InputSystem;
use crate::systems::gameplay_state_system::GameplayStateSystem;


pub struct MusicSound {
    pub ingame_music: Source,
    pub victory_music: Source
}

impl MusicSound {
    pub fn new(context: &mut Context) -> Self {
        MusicSound {
            ingame_music: Source::new(context, "/sounds/ingame_music.mp3").unwrap(),
            victory_music: Source::new(context, "/sounds/victory_music.mp3").unwrap()
        }
    }
}

pub struct Game {
    pub world: World,
    pub music_sound: MusicSound
}

impl Game {
    pub fn new(world: World, context: &mut Context) -> Self {
        Game {
            world,
            music_sound: MusicSound::new(context)
        }
    }

    pub fn play_ingame_music(&mut self) {
        self.music_sound.ingame_music.play().unwrap();
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { context: ctx };
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.push(keycode);
    }
}
