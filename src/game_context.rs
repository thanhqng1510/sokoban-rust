use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use ggez::audio::{Source};
use ggez::audio::SoundSource;
use crate::systems::rendering_system::RenderingSystem;
use crate::resources::input_queue::InputQueue;
use crate::systems::input_system::InputSystem;
use crate::systems::gameplay_state_system::GameplayStateSystem;
use ggez::graphics::Color;
use crate::constant::{MAX_LEVEL, RESOURCE_PREFIX_PATH};
use std::cmp::min;
use std::fs;
use crate::components::{Position, Direction, Renderable, Wall, Box, Player, Spot, Movable, Blocking, Directional};
use crate::resources::game_state::GameState;
use crate::resources::component_template_data::ComponentTemplateData;
use crate::entity_builder::EntityBuilder;


pub struct MusicSound {
    pub ingame_music: Option<Source>,
    pub victory_music: Option<Source>
}

impl MusicSound {
    pub fn new() -> Self {
        MusicSound {
            ingame_music: None,
            victory_music: None
        }
    }
}

pub struct GameContext {
    pub world: World,
    pub music_sound: MusicSound,
    pub background_color: Color
}

impl GameContext {
    pub fn from(world: World) -> Self {
        GameContext {
            world,
            music_sound: MusicSound::new(),
            background_color: Color::new(0.7, 0.7, 0.7, 1.)
        }
    }

    pub fn initialize_level(&mut self, level: u8, context: &mut Context) {
        let level = min(level, MAX_LEVEL);
        let map_string= fs::read_to_string(format!("{}/maps/map_{}.txt", RESOURCE_PREFIX_PATH, level))
            .expect(&format!("Unable to read file. Check if level {} exists!", level));
        self.generate_map(map_string);

        self.music_sound.ingame_music = Some(Source::new(context, format!("/sounds/musics/victory_music_{}.mp3", level)).unwrap());
        self.music_sound.victory_music = Some(Source::new(context, format!("/sounds/musics/victory_music_{}.mp3", level)).unwrap());

        if let Some(ref mut ingame_music) = self.music_sound.ingame_music {
            ingame_music.play().unwrap();
        }
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
        self.world.insert(ComponentTemplateData::default());
    }

    pub fn generate_map(&mut self, map_string: String) {
        let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

        for (y, &row) in rows.iter().enumerate() {
            let columns = row.split(' ').collect::<Vec<_>>();

            for (x, &column) in columns.iter().enumerate() {
                let position = Position { x: x as u8, y: y as u8, z: 0 };

                match column {
                    "." => EntityBuilder::create_floor(&mut self.world, position),
                    "W" => {
                        EntityBuilder::create_wall(&mut self.world, position);
                        EntityBuilder::create_floor(&mut self.world, position);
                    },
                    "P" => {
                        EntityBuilder::create_player(&mut self.world, position, Direction::Down);
                        EntityBuilder::create_floor(&mut self.world, position);
                    },
                    "B" => {
                        EntityBuilder::create_box(&mut self.world, position);
                        EntityBuilder::create_floor(&mut self.world, position);
                    },
                    "S" => {
                        EntityBuilder::create_spot(&mut self.world, position);
                        EntityBuilder::create_floor(&mut self.world, position);
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

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.push(keycode);
    }
}
