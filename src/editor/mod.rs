//! Editor specific stuff

mod bundles;
mod components;
mod events;
mod inputs;
mod states;
mod systems;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
};
use amethyst::utils::application_root_dir;

use events::EditorStateEventReader;

const DISPLAY_CONFIG_PATH: &str = "resources/display_config.ron";
const INPUT_CONFIG_PATH: &str = "resources/editor_bindings_config.ron";

pub fn start_level_editor() -> Result<(), amethyst::Error> {
    let display_config = DisplayConfig::load(&DISPLAY_CONFIG_PATH);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let input_bundle = InputBundle::<(), inputs::EditorInputAction>::new()
        .with_bindings_from_file(INPUT_CONFIG_PATH)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(bundles::EditorBundle)?
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?;

    let mut game = CoreApplication::<_, _, EditorStateEventReader>::build(
        application_root_dir() + "/resources",
        crate::states::LoaderState::new(
            crate::assets::LoadableAsset::new("textures/entities-spritesheet")
                .with_editor_assets("textures/editor-spritesheet"),
            Box::new(self::states::LevelEditorState::new()),
        ),
    )?
    .build(game_data)?;
    game.run();
    Ok(())
}
