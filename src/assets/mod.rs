use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

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

#[derive(Clone)]
pub struct GameAssets {
    pub entities_spritesheet: SpriteSheetHandle,
}

impl GameAssets {
    pub fn entity_sprite(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.entities_spritesheet.clone(),
            sprite_number,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
}
