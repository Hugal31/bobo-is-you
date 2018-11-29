use amethyst::prelude::*;

use crate::assets::GameAssets;
use super::LevelState;

pub struct MenuState {
    assets: GameAssets,
}

impl MenuState {
    pub fn new(assets: GameAssets) -> MenuState {
        MenuState {
            assets,
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MenuState {

    fn update(&mut self, _data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        Trans::Push(Box::new(LevelState::new(self.assets.clone())))
    }
}
