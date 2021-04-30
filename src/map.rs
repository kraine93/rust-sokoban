use crate::components::{BoxColour, Position};
use crate::entities::*;
use specs::World;

pub fn load_map(world: &mut World, map_string: String) {
    let rows = map_string
        .trim()
        .split('\n')
        .map(|r| r.trim())
        .collect::<Vec<&str>>();

    for (y, row) in rows.iter().enumerate() {
        let columns = row.split(' ').collect::<Vec<&str>>();

        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };

            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "BB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Blue);
                }
                "BS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Blue);
                }
                "RB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Red);
                }
                "RS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Red);
                }
                "N" => (),
                c => panic!("Unrecognised map item {}", c),
            }
        }
    }
}
