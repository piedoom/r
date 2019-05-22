use crate::node::*;
use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use crate::systems::Catcher;

pub struct Sprites {
    pub arrows: SpriteSheetHandle,
    pub arrows_line: SpriteSheetHandle,
}

impl Sprites {
    pub fn new(world: &mut World) -> Self {
        Self {
            arrows: Game::load_sprite_sheet(world, "textures/arrows.png"),
            arrows_line: Game::load_sprite_sheet(world, "textures/arrows_line.png")
        }
    }
}

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        // create an alias for the world var since it is so commonly used
        let world = data.world;

        // Load sprites and add as a resource
        let sprites = Sprites::new(world);
        world.add_resource(sprites);

        // Setup our camera
        Self::initialize_camera(world);

        // setup other game stuff
        Self::initialize_game(world);
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

    fn initialize_game(world: &mut World) {
        // setup catcher arrows

        // get sprite sheet 
        // TODO: use resource
        let sprites = Self::load_sprite_sheet(world, "textures/arrows_line.png");

        // loop to create all arrows
        for i in 0..4 {
            let sprite_render = SpriteRender {
                sprite_sheet: sprites.clone(),
                // This is somewhat breakable. Should probably use an enum conversion in the future.
                sprite_number: i,
            };
            world.create_entity()
                .with(
                    Transform::from(Vector3::new(16.0f64 + (i as f64 * 16.0),50.0,0.0))
                )
                .with(Catcher{direction: NodeDirection::from(i)})
                .with(sprite_render)
                .build();
        }
    }

    pub fn initialize_camera(world: &mut World) {
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0, 100.0, 0.0, 100.0,
            )))
            .with(Transform::from(Vector3::new(0.0,0.0,1.0)))
            .build();
    }
}

