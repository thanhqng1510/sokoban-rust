use ggez::{Context, graphics};
use specs::{System, ReadStorage, Join, Read};
use crate::components::Renderable;
use ggez::graphics::{Image, DrawParam, Color};
use ggez::nalgebra as na;
use crate::constant::{TILE_SIZE, BACKGROUND_COLOR};
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
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        graphics::clear(self.context, graphics::Color::new(
            *BACKGROUND_COLOR.get("r").unwrap(),
            *BACKGROUND_COLOR.get("g").unwrap(),
            *BACKGROUND_COLOR.get("b").unwrap(),
            *BACKGROUND_COLOR.get("a").unwrap())
        );

        let (game_state, renderables) = data;
        let mut rendering_data = (&renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.position.z);

        for renderable in rendering_data {
            let image = Image::new(self.context, renderable.resource_path).unwrap();
            let x = renderable.position.x as f32 * TILE_SIZE;
            let y = renderable.position.y as f32 * TILE_SIZE;

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).unwrap();
        }

        self.draw_text(&game_state.gameplay_state.to_string(), 525., 80.);
        self.draw_text(&game_state.moves_count.to_string(), 525., 100.);

        graphics::present(self.context).unwrap();
    }
}
