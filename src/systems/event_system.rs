use crate::audio::AudioStore;
use crate::{
    components::*,
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    resources::EventQueue,
};
use specs::{Entities, Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct EventSystem {}

impl<'a> System<'a> for EventSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut audio_store, entities, boxes, box_spots, positions) = data;

        let mut new_events = Vec::new();

        // Loop all the events in the queue
        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);
            match event {
                Event::PlayerHitObstacle => {
                    audio_store.play("wall");
                }
                Event::EntityMoved(EntityMoved { id }) => {
                    // Check the entity is a box
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spot_positions: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|t| ((t.1.x, t.1.y), t.0))
                                .collect();

                        // Check the box position to see if it is on a box spot
                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            if let Some(box_spot) =
                                box_spot_positions.get(&(box_position.x, box_position.y))
                            {
                                // Push a new event checking if the colours match
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: box_spot.colour == the_box.colour,
                                }))
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    // play sound
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };
                    audio_store.play(sound);
                }
            }
        }
        event_queue.events.append(&mut new_events);
    }
}
