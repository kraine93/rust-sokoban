use specs::{Component, NullStorage, VecStorage, World, WorldExt};
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq)]
pub enum BoxColour {
    Blue,
    Red,
}

impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            Self::Blue => "blue",
            Self::Red => "red",
        })?;
        Ok(())
    }
}

pub enum RenderableType {
    Static,
    Animated,
}

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    paths: Vec<String>,
}

impl Renderable {
    pub fn new_static(path: String) -> Self {
        Renderable { paths: vec![path] }
    }

    pub fn new_animated(paths: Vec<String>) -> Self {
        Renderable { paths }
    }

    pub fn kind(&self) -> RenderableType {
        match self.paths.len() {
            0 => panic!("Invalid renderable"),
            1 => RenderableType::Static,
            _ => RenderableType::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> &String {
        &self.paths[path_index % self.paths.len()]
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Movable>();
    world.register::<Immovable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}
