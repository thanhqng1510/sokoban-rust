use ggez::{Context, graphics};
use specs::{System, ReadStorage, Join, ReadExpect};
use crate::components::Renderable;
use ggez::graphics::{Image, DrawParam, Color};
use ggez::nalgebra as na;
use strfmt::strfmt;
use crate::constant::TILE_SIZE;
use crate::resources::game_state::GameState;
use crate::game_context::GameContext;
use crate::resources::level_data::LevelData;


pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
    pub game_context: &'a GameContext
}

impl<'a> RenderingSystem<'a> {
    pub fn from(context: &'a mut Context, game_context: &'a GameContext) -> Self {
        RenderingSystem { context, game_context }
    }

    pub fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text);
        let destination = na::Point2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimension = na::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimension, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        ).unwrap();
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadExpect<'a, LevelData>,
        ReadExpect<'a, GameState>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (level_data, game_state, renderables) = data;

        graphics::clear(self.context, level_data.background_color);

        let mut rendering_data = (&renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.position.z);

        for renderable in rendering_data {
            let image_path = strfmt(renderable.resource_template_path, &renderable.resource_template_data).unwrap();
            let image = Image::new(self.context, image_path).unwrap();

            let x = renderable.position.x as f32 * TILE_SIZE;
            let y = renderable.position.y as f32 * TILE_SIZE;

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).unwrap();
        }

        self.draw_text(&game_state.gameplay_state.to_string(), 600., 80.);
        self.draw_text(&game_state.moves_count.to_string(), 600., 100.);

        graphics::present(self.context).unwrap();
    }
}
