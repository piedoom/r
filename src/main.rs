use amethyst::core::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::utils::application_root_dir;

use amethyst::input::InputBundle;
use amethyst::prelude::*;

mod game;
mod node;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root_dir = application_root_dir()?;
    let assets_dir = root_dir.join("resources");

    let path = assets_dir.join("display_config.ron");
    let config = DisplayConfig::load(&path);
    let binding_path = assets_dir.join("bindings_config.ron");
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::SpawnerSystem::default(), "spawner_system", &[])
        .with(systems::NodeSystem, "node_system", &["input_system"]);


    let mut game = Application::new(assets_dir, game::Game, game_data)?;
    game.run();
    Ok(())
}
