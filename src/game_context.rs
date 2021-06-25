use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{KeyCode, KeyMods, quit};
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
        sound_lib.load_music(context, level);
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
        self.world.insert(InputQueue::new());
        self.world.insert(GameState::default());
        self.world.insert(SoundLibrary::new());
        self.world.insert(GameVars::default());
        self.world.insert(Timer::new());
    }

    pub fn generate_map(&mut self, map_string: String) {
        let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

        for (_, &row) in rows.iter().enumerate() {
            let columns = row.split(';').map(|x| x.trim()).collect::<Vec<_>>();
            let entity_data = columns[0].split(' ').collect::<Vec<_>>();

            for (_, &column) in columns.iter().enumerate().filter(|&(i, _)| i != 0) {
                let mut position = Position { x: 0, y: 0, z: 0 };
                for(i, &pos) in column.split(',').collect::<Vec<_>>().iter().enumerate() {
                    match i {
                        0 => position.y = pos.parse::<u8>().unwrap(),
                        1 => position.x = pos.parse::<u8>().unwrap(),
                        _ => panic!("Dimension of position exceeded 2.")
                    }
                }

                match entity_data[0] {
                    "floor" => {
                        let floor_type: FloorType = match entity_data[1] {
                            "clean" => FloorType::Clean,
                            "gravel" => FloorType::Gravel,
                            c => panic!("Unrecognized FloorType {}.", c)
                        };
                        let floor_material: FloorMaterial = match entity_data[2] {
                            "concrete" => FloorMaterial::Concrete,
                            "dirt" => FloorMaterial::Dirt,
                            "grass" => FloorMaterial::Grass,
                            "sand" => FloorMaterial::Sand,
                            c => panic!("Unrecognized FloorMaterial {}.", c)
                        };
                        EntityBuilder::create_floor(&mut self.world, position, floor_type, floor_material);
                    },
                    "wall" => {
                        let wall_shape: WallShape = match entity_data[1] {
                            "square" => WallShape::Square,
                            "round" => WallShape::Round,
                            c => panic!("Unrecognized WallShape {}.", c)
                        };
                        let wall_color: WallColor = match entity_data[2] {
                            "beige" => WallColor::Beige,
                            "black" => WallColor::Black,
                            "brown" => WallColor::Brown,
                            "gray" => WallColor::Gray,
                            c => panic!("Unrecognized WallColor {}.", c)
                        };
                        EntityBuilder::create_wall(&mut self.world, position, wall_shape, wall_color);
                    },
                    "box" => {
                        let box_type: BoxType = match entity_data[1] {
                            "dark" => BoxType::Dark,
                            "bright" => BoxType::Bright,
                            c => panic!("Unrecognized BoxType {}.", c)
                        };
                        let box_color: BoxSpotColor = match entity_data[2] {
                            "beige" => BoxSpotColor::Beige,
                            "black" => BoxSpotColor::Black,
                            "blue" => BoxSpotColor::Blue,
                            "brown" => BoxSpotColor::Brown,
                            "gray" => BoxSpotColor::Gray,
                            "purple" => BoxSpotColor::Purple,
                            "red" => BoxSpotColor::Red,
                            "yellow" => BoxSpotColor::Yellow,
                            c => panic!("Unrecognized BoxColor {}.", c)
                        };
                        EntityBuilder::create_box(&mut self.world, position, box_type, box_color);
                    },
                    "spot" => {
                        let spot_color: BoxSpotColor = match entity_data[1] {
                            "beige" => BoxSpotColor::Beige,
                            "black" => BoxSpotColor::Black,
                            "blue" => BoxSpotColor::Blue,
                            "brown" => BoxSpotColor::Brown,
                            "gray" => BoxSpotColor::Gray,
                            "purple" => BoxSpotColor::Purple,
                            "red" => BoxSpotColor::Red,
                            "yellow" => BoxSpotColor::Yellow,
                            c => panic!("Unrecognized SpotColor {}.", c)
                        };
                        EntityBuilder::create_spot(&mut self.world, position, spot_color);
                    },
                    "char" => {
                        EntityBuilder::create_player(&mut self.world, position, Direction::Down);
                    },
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
