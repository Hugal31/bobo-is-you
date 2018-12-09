use amethyst::assets::{AssetStorage, Loader, ProgressCounter};
use amethyst::ecs::Resources;
use amethyst::prelude::*;
use amethyst::renderer::{
    PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureHandle,
    TextureMetadata,
};

use super::MenuState;
use crate::assets::GameAssets;
use crate::events::BoboStateEvent;

#[derive(Default)]
pub struct LoaderState {
    /// Progress tracker.
    progress: ProgressCounter,
}

impl LoaderState {
    pub fn new() -> LoaderState {
        LoaderState::default()
    }

    fn load_textured_spritesheet(
        &mut self,
        name: &str,
        resources: &Resources,
    ) -> SpriteSheetHandle {
        let texture = self.load_texture(&format!("{}.png", name), resources);
        self.load_sprite_sheet(&format!("{}.ron", name), texture, resources)
    }

    fn load_texture(&mut self, path: &str, resources: &Resources) -> TextureHandle {
        let loader = resources.fetch::<Loader>();
        let texture_storage = resources.fetch::<AssetStorage<Texture>>();
        loader.load(
            path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            &mut self.progress,
            &texture_storage,
        )
    }

    fn load_sprite_sheet(
        &mut self,
        path: &str,
        texture: TextureHandle,
        resources: &Resources,
    ) -> SpriteSheetHandle {
        /*let texture_id = self.texture_ids;
        let mut material_texture_set = resources.fetch_mut::<MaterialTextureSet>();
        material_texture_set.insert(texture_id, texture);*/
        let loader = resources.fetch::<Loader>();
        let spritesheet_storage = resources.fetch::<AssetStorage<SpriteSheet>>();
        loader.load(
            path,
            SpriteSheetFormat,
            texture,
            &mut self.progress,
            &spritesheet_storage,
        )
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, BoboStateEvent> for LoaderState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        world.add_resource(GameAssets {
            entities_spritesheet: self
                .load_textured_spritesheet("textures/entities-spritesheet", &world.res),
        });
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        data.data.update(data.world);

        if self.progress.is_complete() {
            debug!("Loading complete!");
            Trans::Switch(Box::new(MenuState::new()))
        } else {
            Trans::None
        }
    }
}
