//! creates nodes according to a song
use crate::game::Sprites;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::{prelude::*, Join, Read, ReadStorage, Resources, System, WriteStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet, SpriteSheetHandle};
use crate::node::Node;

pub struct SpawnerSystem {
    origin: (f64, f64),
}

impl Default for SpawnerSystem {
    fn default() -> Self {
        Self {
            origin: (50.0, 80.0),
        }
    }
}

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        ReadExpect<'s, Sprites>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Node>,
    );

    fn run(
        &mut self,
        (sprites, entities, mut transform_storage, mut sprite_render_storage, mut node_storage): Self::SystemData,
    ) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprites.arrows.clone(),
            sprite_number: 0,
        };

        entities
            .build_entity()
            .with(sprite_render, &mut sprite_render_storage)
            .with(
                Transform::from(Vector3::new(self.origin.0, self.origin.1, 0.0)),
                &mut transform_storage,
            )
            .with(Node, &mut node_storage)
            .build();
    }
}
