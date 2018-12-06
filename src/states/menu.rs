use amethyst::prelude::*;

use super::LevelLoaderState;
use crate::events::BoboStateEvent;

pub struct MenuState {}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {}
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, BoboStateEvent> for MenuState {
    fn update(&mut self, _data: StateData<GameData>) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        Trans::Push(Box::new(LevelLoaderState::for_level("levels/level1.ron")))
    }
}
