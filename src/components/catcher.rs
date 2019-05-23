//! Catcher and Catchers prefab
use amethyst::{
    ecs::{
        Storage, DenseVecStorage, Component
    }
};
use crate::util::Direction;

pub struct Catcher {
    pub direction: Direction,
}

impl Catcher {
    // pub fn load_prefab(world: &mut World) -> Handle<Prefab<Catcher>> {
    //     world.exec(|loader: PrefabLoader<'_, Catcher>| {
    //         loader.load(
    //             "prefabs/catcher.ron",
    //             RonFormat,
    //             (),
    //             (),
    //         )
    //     })
    // }
}

impl Component for Catcher {
    type Storage = DenseVecStorage<Self>;
}

// use amethyst::{
//     assets::{PrefabLoader, RonFormat, Handle, Prefab, ProgressCounter, PrefabData},
//     derive::PrefabData,
//     renderer::{GraphicsPrefab, PosNormTex, ObjFormat, TextureFormat},
//     ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
//     Error,
//     prelude::*,
// };
// use serde::{Deserialize, Serialize};
// use crate::util::{Direction, DirectionData};

// #[derive(Default, Deserialize, Serialize, PrefabData)]
// #[serde(default)]
// #[serde(deny_unknown_fields)]
// pub struct CatcherPrefabData {
//     pub direction: Option<DirectionData>,
//     pub graphics: Option<GraphicsPrefab<Vec<PosNormTex>, ObjFormat, TextureFormat>>,
// }

