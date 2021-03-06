use crate::components::*;
use crate::constants::*;
use crate::events::{EntityMoved, Event};
use crate::resources::{EventQueue, Gameplay, InputQueue};
use ggez::event::KeyCode;
use specs::{join::Join, world::Index, Entities, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, EventQueue>,
        Write<'a, Gameplay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut input_queue,
            mut event_queue,
            mut gameplay,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                // Movables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect();

                // Immovables
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect();

                // Iterate over each element in the direction until the end of the map
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<u8>>()
                } else {
                    (end..=start).rev().collect::<Vec<u8>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // Try to find a movable object at the position, and if so, add it to the move list
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        // Try to find an immovable object at the location, and if so, clear all movements
                        None => match immov.get(&pos) {
                            Some(_id) => {
                                to_move.clear();
                                event_queue.events.push(Event::PlayerHitObstacle);
                            }
                            None => break,
                        },
                    }
                }
            }
        }

        if !to_move.is_empty() {
            gameplay.moves_count += 1;
        }

        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
            event_queue
                .events
                .push(Event::EntityMoved(EntityMoved { id }));
        }
    }
}
