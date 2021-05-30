use ggez::{Context, graphics};
use specs::{System, ReadStorage, Join};
use crate::components::{Position, Renderable};
use ggez::graphics::{Image, DrawParam};
use ggez::nalgebra as na;
use crate::entities::TILE_WIDTH;


pub struct RenderingSystem<'a> {
    context: &'a mut Context
}

impl<'a> RenderingSystem<'a> {
    pub fn from(ctx: &'a mut Context) -> Self {
        RenderingSystem { context: ctx }
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.));

        let (positions, renderables) = data;
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderables) in rendering_data.iter() {
            let image = Image::new(self.context, &renderables.path).unwrap();
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).unwrap();
        }

        graphics::present(self.context).unwrap();
    }
}
