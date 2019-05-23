use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Node;

impl Component for Node {
    type Storage = DenseVecStorage<Self>;
}