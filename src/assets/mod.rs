use amethyst::assets::{AssetStorage, Loader, ProgressCounter};
use amethyst::ecs::{Resources, World};
use amethyst::renderer::{
    PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
    TextureHandle, TextureMetadata,
};

pub const ENTITY_SPRITE_BOBO: usize = 0;
pub const ENTITY_SPRITE_WALL: usize = 1;
pub const ENTITY_SPRITE_FLAG: usize = 10;
pub const ENTITY_SPRITE_INST_IS: usize = 2;
pub const ENTITY_SPRITE_INST_BOBO: usize = 3;
pub const ENTITY_SPRITE_INST_WALL: usize = 4;
pub const ENTITY_SPRITE_INST_YOU: usize = 5;
pub const ENTITY_SPRITE_INST_STOP: usize = 6;
pub const ENTITY_SPRITE_INST_PUSH: usize = 7;
pub const ENTITY_SPRITE_INST_FLAG: usize = 8;
pub const ENTITY_SPRITE_INST_WIN: usize = 9;

// TODO Search if we can do better with prefabs
pub struct LoadableAsset {
    editor_assets: Option<String>,
    game_assets: String,
}

impl LoadableAsset {
    pub fn new(game_assets: impl Into<String>) -> Self {
        LoadableAsset {
            editor_assets: None,
            game_assets: game_assets.into(),
        }
    }

    pub fn with_editor_assets(self, editor_assets: impl Into<String>) -> Self {
        LoadableAsset {
            editor_assets: Some(editor_assets.into()),
            ..self
        }
    }

    pub fn load(&self, world: &mut World, progress: &mut ProgressCounter) {
        {
            let entities_sprite_sheet =
                load_textured_spritesheet(&self.game_assets, &mut world.res, progress);
            world.add_resource(GameAssets {
                entities_sprite_sheet,
            });
        }

        if let Some(editor_assets) = &self.editor_assets {
            let sprite_sheet = load_textured_spritesheet(editor_assets, &mut world.res, progress);
            world.add_resource(EditorAssets { sprite_sheet });
        }
    }
}

#[derive(Clone)]
pub struct GameAssets {
    entities_sprite_sheet: SpriteSheetHandle,
}

impl GameAssets {
    pub fn entity_sprite(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.entities_sprite_sheet.clone(),
            sprite_number,
        }
    }
}

pub struct EditorAssets {
    sprite_sheet: SpriteSheetHandle,
}

impl EditorAssets {
    pub const SPRITE_CURSOR: usize = 0;

    pub fn sprite(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.sprite_sheet.clone(),
            sprite_number,
        }
    }
}

fn load_textured_spritesheet(
    name: &str,
    resources: &Resources,
    progress: &mut ProgressCounter,
) -> SpriteSheetHandle {
    let texture = load_texture(&format!("{}.png", name), resources, progress);
    load_sprite_sheet(&format!("{}.ron", name), texture, resources, progress)
}

fn load_texture(
    path: &str,
    resources: &Resources,
    progress: &mut ProgressCounter,
) -> TextureHandle {
    let loader = resources.fetch::<Loader>();
    let texture_storage = resources.fetch::<AssetStorage<Texture>>();
    loader.load(
        path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        progress,
        &texture_storage,
    )
}

fn load_sprite_sheet(
    path: &str,
    texture: TextureHandle,
    resources: &Resources,
    progress: &mut ProgressCounter,
) -> SpriteSheetHandle {
    let loader = resources.fetch::<Loader>();
    let spritesheet_storage = resources.fetch::<AssetStorage<SpriteSheet>>();
    loader.load(
        path,
        SpriteSheetFormat,
        texture,
        progress,
        &spritesheet_storage,
    )
}
