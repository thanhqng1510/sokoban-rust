use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{KeyCode, KeyMods, quit};
use ggez::audio::{Source};
use ggez::audio::SoundSource;
use crate::systems::rendering_system::RenderingSystem;
use crate::systems::input_system::InputSystem;
use crate::systems::gameplay_state_system::GameplayStateSystem;
use crate::constant::{RESOURCE_PREFIX_PATH, MAX_LEVEL};
use crate::components::{Position, Direction, Renderable, Wall, Box, Player, Spot, Movable, Blocking, Directional, FloorType, FloorMaterial, WallColor, WallShape, BoxSpotColor, BoxType};
use crate::resources::input_queue::InputQueue;
use crate::resources::game_state::GameState;
use crate::resources::sound_library::SoundLibrary;
use crate::resources::game_vars::GameVars;
use crate::resources::timer::Timer;
use crate::entity_builder::EntityBuilder;
use std::fs;
use std::cmp::min;


pub struct GameContext {
    pub world: World,
}

impl GameContext {
    pub fn from(world: World) -> Self {
        GameContext { world }
    }

    pub fn initialize_level(&mut self, level: u8, context: &mut Context) {
        self.world.delete_all();
        self.world.write_resource::<InputQueue>().clear();
        self.world.write_resource::<GameState>().clear();

        let level = min(level, MAX_LEVEL);
        self.world.write_resource::<GameVars>().current_level = level;

        let map_string= fs::read_to_string(format!("{}/maps/map_{}.txt", RESOURCE_PREFIX_PATH, level)).unwrap();
        self.generate_map(map_string);

        let mut sound_lib = self.world.write_resource::<SoundLibrary>();
        sound_lib.music_sound.ingame_music = Some(Source::new(context, format!("/sounds/musics/ingame_music_{}.wav", level)).unwrap());
        sound_lib.music_sound.victory_music = Some(Source::new(context, format!("/sounds/musics/victory_music_{}.wav", level)).unwrap());

        if let Some(ref mut ingame_music) = sound_lib.music_sound.ingame_music { ingame_music.play().unwrap(); }
    }

    pub fn restart_level(&mut self) {
        self.world.delete_all();
        self.world.write_resource::<InputQueue>().clear();
        self.world.write_resource::<GameState>().clear();

        let level = self.world.read_resource::<GameVars>().current_level;

        let map_string= fs::read_to_string(format!("{}/maps/map_{}.txt", RESOURCE_PREFIX_PATH, level)).unwrap();
        self.generate_map(map_string);

        let mut sound_lib = self.world.write_resource::<SoundLibrary>();
        if let Some(ref mut victory_music) = sound_lib.music_sound.victory_music {
            if victory_music.playing() { victory_music.stop(); }
        }
        if let Some(ref mut ingame_music) = sound_lib.music_sound.ingame_music { ingame_music.play().unwrap(); }
    }

    pub fn register_components(&mut self) {
        self.world.register::<Renderable>();
        self.world.register::<Wall>();
        self.world.register::<Player>();
        self.world.register::<Box>();
        self.world.register::<Spot>();
        self.world.register::<Movable>();
        self.world.register::<Blocking>();
        self.world.register::<Directional>();
    }

    pub fn register_resources(&mut self) {
        self.world.insert(InputQueue::default());
        self.world.insert(GameState::default());
        self.world.insert(SoundLibrary::default());
        self.world.insert(GameVars::default());
        self.world.insert(Timer::new());
    }

    pub fn generate_map(&mut self, map_string: String) {
        let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

        for (y, &row) in rows.iter().enumerate() {
            let columns = row.split(' ').collect::<Vec<_>>();

            for (x, &column) in columns.iter().enumerate() {
                let position = Position { x: x as u8, y: y as u8, z: 0 };

                match column {
                    "." => EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand),
                    "W" => {
                        EntityBuilder::create_wall(&mut self.world, position, WallColor::Gray, WallShape::Square);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "P" => {
                        EntityBuilder::create_player(&mut self.world, position, Direction::Down);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "B" => {
                        EntityBuilder::create_box(&mut self.world, position, BoxType::Bright, BoxSpotColor::Red);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "S" => {
                        EntityBuilder::create_spot(&mut self.world, position, BoxSpotColor::Red);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "N" => (),
                    c => panic!("Unrecognized map item {}", c)
                }
            }
        }
    }
}

impl event::EventHandler for GameContext {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut is = InputSystem::new();
        is.run_now(&self.world);
        drop(is);

        let mut gss = GameplayStateSystem::new();
        gss.run_now(&self.world);
        drop(gss);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rs = RenderingSystem::from(ctx, self);
        rs.run_now(&self.world);
        drop(rs);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape => quit(ctx),
            KeyCode::R => self.restart_level(),
            _ => {
                let mut input_queue = self.world.write_resource::<InputQueue>();
                input_queue.push(keycode);
            }
        }
    }
}
