use crate::components::*;
use crate::resources::{Gameplay, GameplayState};
use specs::{join::Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct GameStateSystem {}

impl<'a> System<'a> for GameStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots) = data;

        // Get all boxes
        let box_positions: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect();

        // Check all box spots to see if there is a box at the same position
        for (_box_spot, position) in (&box_spots, &positions).join() {
            if !box_positions.contains_key(&(position.x, position.y)) {
                gameplay_state.state = GameplayState::Playing;
                return;
            }
        }

        gameplay_state.state = GameplayState::Won;
    }
}
