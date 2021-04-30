use crate::components::*;
use crate::constants::TILE_WIDTH;
use crate::resources::{Gameplay, Time};
use ggez::{
    graphics,
    graphics::{spritebatch::SpriteBatch, Color, DrawParam, Image},
    nalgebra as na, timer, Context,
};
use itertools::Itertools;
use specs::{join::Join, Read, ReadStorage, System};
use std::collections::HashMap;
use std::time::Duration;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay_state, time, positions, renderables) = data;

        // Clear the screen (set the background colour)
        graphics::clear(self.context, Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all renderables and sort by z-index position (for layering elements)
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        // Iterate through renderables and add to the batches
        for (position, renderable) in rendering_data.iter() {
            let image_path = self.get_image(renderable, time.delta);

            let calc_pos = |num: f32| num * TILE_WIDTH;
            let x = calc_pos(position.x as f32);
            let y = calc_pos(position.y as f32);

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));

            rendering_batches
                .entry(position.z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_params);
        }

        // Iterate the batches and render
        for (_z, batch) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in batch {
                let image = Image::new(self.context, image_path).expect("Expected image");

                let mut sprite_batch = SpriteBatch::new(image);
                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("Expected render");
            }
        }

        self.draw_text(&gameplay_state.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay_state.moves_count.to_string(), 525.0, 100.0);
        let fps = format!("FPS: {}", timer::fps(self.context));
        self.draw_text(&fps.to_string(), 525.0, 120.0);

        graphics::present(self.context).expect("Expected to present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let colour = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = na::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, colour);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("Expected draw text");
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableType::Static => 0,
            RenderableType::Animated => ((delta.as_millis() % 1000) / 250) as usize,
        };

        renderable.path(path_index).to_string()
    }
}
