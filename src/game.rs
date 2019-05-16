use crate::node::*;
use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::core::Transform;

pub struct Sprites {
    pub arrows: SpriteSheetHandle
}

impl Sprites {
    pub fn new(world: &mut World) -> Self {
        Self {
            arrows: Game::load_sprite_sheet(world, "textures/arrows.png")
        }
    }
}

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprites = Sprites::new(world);
        world.add_resource(sprites);

        Self::initialize_camera(world);
    }
}

impl Game {
    pub fn load_sprite_sheet(world: &mut World, path: &str) -> SpriteSheetHandle {
        // Load the sprite sheet necessary to render the graphics.
        // The texture is the pixel data
        // `texture_handle` is a cloneable reference to the texture
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                path,
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}.{}", path, "ron"), // Here we load the associated ron file
            SpriteSheetFormat,
            texture_handle, // We pass it the handle of the texture we want it to use
            (),
            &sprite_sheet_store,
        )
    }

    pub fn initialize_camera(world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0, 100.0, 0.0, 100.0,
            )))
            .with(transform)
            .build();
    }
}

