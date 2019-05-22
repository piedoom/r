use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},

};

pub enum NodeDirection {
    Left,
    Up,
    Down,
    Right,
}

impl Default for NodeDirection {
    fn default() -> Self {
        NodeDirection::Left
    }
}

impl NodeDirection {
    pub fn to_usize(&self) -> usize {
        match self {
            NodeDirection::Left => 0,
            NodeDirection::Up => 1,
            NodeDirection::Down => 2,
            NodeDirection::Right => 3,
        }
    }
}

impl From<usize> for NodeDirection {
    fn from(n: usize) -> Self {
        match n {
            0 => NodeDirection::Left,
            1 => NodeDirection::Up,
            2 => NodeDirection::Down,
            3 => NodeDirection::Right,
            _ => unreachable!()
        }
    }
}

pub struct Node;

impl Component for Node {
    type Storage = DenseVecStorage<Self>;
}