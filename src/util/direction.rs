use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{Entity, WriteStorage, DenseVecStorage, Component},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Left
    }
}

impl Direction {
    pub fn to_usize(&self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        }
    }
}

impl From<usize> for Direction {
    fn from(n: usize) -> Self {
        match n {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => unreachable!()
        }
    }
}

impl From<DirectionData> for Direction {
    fn from(d: DirectionData) -> Self {
        Direction::from(d.0)
    }
}

// Since we can't derive PrefabData for an enum, we need to do this.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct DirectionData(pub usize);

impl Component for DirectionData {
    type Storage = DenseVecStorage<Self>;
}