use crate::components::*;
use crate::util::Direction;
use amethyst::assets::{AssetStorage, Loader, Handle, Prefab, PrefabLoader, RonFormat, PrefabData};
use amethyst::ecs::prelude::{Component, DenseVecStorage, SystemData};
use amethyst::prelude::*;
use amethyst::{
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Texture,
    },
};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::core::transform::Parent;
use serde::{Serialize, Deserialize};
use crate::systems::CatcherSystem;

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
    fn load_sprite_sheet(world: &mut World, path: &str) -> SpriteSheetHandle {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                path,
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}.{}", path, "ron"),
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    }

    fn initialize_game(world: &mut World) {
        let catchers = world.create_entity().with(Transform::default()).build();

        let columns = [Direction::Left, Direction::Up, Direction::Down, Direction::Right];
        for column in columns.iter() {
            world.create_entity()
                .with(Parent::new(catchers))
                .with(Catcher{direction: column.clone()})
                .build();
        }

        // attach our prefab to our parent entity
        /* world.create_entity()
            .with(catchers_prefab_handle.clone())
            .with(Parent::new(catchers))
            .build();
        
        */
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
                .with(Catcher{direction: Direction::from(i)})
                .with(sprite_render)
                .build();
        }
    }

    pub fn initialize_camera(world: &mut World) {
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0, 100.0, 0.0, 100.0, 0.1, 2000.0,
            )))
            .with(Transform::from(Vector3::new(0.0,0.0,1.0)))
            .build();
    }
}

