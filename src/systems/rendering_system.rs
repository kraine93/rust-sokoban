use crate::components::*;
use crate::constants::TILE_WIDTH;
use ggez::{
    graphics,
    graphics::{Color, DrawParam, Image},
    nalgebra as na, Context,
};
use specs::{join::Join, ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Clear the screen (set the background colour)
        graphics::clear(self.context, Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all renderables and sort by z-index position (for layering elements)
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through renderables and draw the images in the corrext position
        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.context, &renderable.path).expect("Expected image");

            let calc_pos = |num: f32| num * TILE_WIDTH;
            let x = calc_pos(position.x as f32);
            let y = calc_pos(position.y as f32);

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("Expected render");
        }

        graphics::present(self.context).expect("Expected to present");
    }
}
