use amethyst::prelude::*;

use super::LevelLoaderState;

pub struct MenuState {}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {}
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MenuState {
    fn update(&mut self, _data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        Trans::Push(Box::new(LevelLoaderState::for_level("levels/level1.ron")))
    }
}
