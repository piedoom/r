use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

mod game;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root_dir = application_root_dir()?;
    let assets_dir = root_dir.join("resources");

    let path = assets_dir.join("display_config.ron");
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let mut game = Application::new(assets_dir, game::Game, game_data)?;
    game.run();
    Ok(())
}
