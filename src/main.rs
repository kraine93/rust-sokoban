use ggez::{
    conf, event,
    event::{KeyCode, KeyMods},
    timer, Context, GameResult,
};
use specs::RunNow;
use specs::{World, WorldExt};
use std::path;

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod map;
mod resources;
mod systems;

use components::*;
use map::*;
use resources::*;
use systems::*;

struct Game {
    world: World,
}

impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        {
            let mut es = EventSystem {};
            es.run_now(&self.world);
        }

        {
            let mut gs = GameStateSystem {};
            gs.run_now(&self.world);
        }

        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context)
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;

    audio::initialize_sounds(&mut world, context);

    let game = &mut Game { world };

    event::run(context, event_loop, game)
}
