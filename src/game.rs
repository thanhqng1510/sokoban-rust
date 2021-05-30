use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use crate::systems::rendering_system::RenderingSystem;
use ggez::event::{KeyCode, KeyMods};
use crate::resources::input_queue::InputQueue;
use crate::systems::input_system::InputSystem;
use crate::systems::gameplay_state_system::GameplayStateSystem;


pub struct Game {
    pub world: World
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
            let mut rs = RenderingSystem::from(ctx);
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.push(keycode);
    }
}
