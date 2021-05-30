use ggez::{Context, graphics};
use specs::{System, ReadStorage, Join, Read};
use crate::components::{Position, Renderable};
use ggez::graphics::{Image, DrawParam, Color};
use ggez::nalgebra as na;
use crate::entities::TILE_WIDTH;
use crate::resources::game_state::GameState;


pub struct RenderingSystem<'a> {
    context: &'a mut Context
}

impl<'a> RenderingSystem<'a> {
    pub fn from(ctx: &'a mut Context) -> Self {
        RenderingSystem { context: ctx }
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
        Read<'a, GameState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        graphics::clear(self.context, graphics::Color::new(0.7, 0.7, 0.7, 1.));

        let (game_state, positions, renderables) = data;
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data {
            let image = Image::new(self.context, &renderable.path).unwrap();
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).unwrap();
        }

        self.draw_text(&game_state.gameplay_state.to_string(), 525., 80.);
        self.draw_text(&game_state.moves_count.to_string(), 525., 100.);

        graphics::present(self.context).unwrap();
    }
}
