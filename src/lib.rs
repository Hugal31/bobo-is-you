extern crate amethyst;
extern crate serde;
#[macro_use]
extern crate log;

mod assets;
mod bundle;
mod components;
mod inputs;
mod prefabs;
mod states;
mod systems;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage, ALPHA,
};

const DISPLAY_CONFIG_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/display_config.ron");
const INPUT_CONFIG_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/bindings_config.ron");

pub fn start_game() -> Result<(), amethyst::Error> {
    let display_config = DisplayConfig::load(&DISPLAY_CONFIG_PATH);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let input_bundle =
        InputBundle::<(), inputs::InputAction>::new().with_bindings_from_file(INPUT_CONFIG_PATH)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(bundle::BoboIsYouBundle)?
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?;

    let mut game =
        Application::build("./resources", states::StartState::new())?.build(game_data)?;
    game.run();
    Ok(())
}