use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

#[derive(Clone)]
pub struct GameAssets {
    pub entities_spritesheet: SpriteSheetHandle,
}

impl GameAssets {
    pub fn character_sprite(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.entities_spritesheet.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
}
