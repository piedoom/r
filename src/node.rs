use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},

};

pub enum NodeDirection {
    Up,
    Down,
    Left,
    Right,
}


impl NodeDirection {
    fn to_usize(&self) -> usize {
        match self {
            NodeDirection::Up => 0,
            NodeDirection::Down => 1,
            NodeDirection::Left => 2,
            NodeDirection::Right => 3,
        }
    }
}

pub struct Node;

impl Component for Node {
    type Storage = DenseVecStorage<Self>;
}