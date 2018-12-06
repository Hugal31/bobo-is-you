use amethyst::prelude::*;

use super::LevelLoaderState;
use crate::events::BoboStateEvent;

pub struct MenuState {
    level_number: u32,
}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState { level_number: 1 }
    }

    /// Returns the path of the level, if it exists.
    fn level_path(&self) -> Option<String> {
        // Fixme: how to get the resource folder ? + check if exists
        Some(format!("levels/level{}.ron", self.level_number))
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, BoboStateEvent> for MenuState {
    fn on_resume(&mut self, _data: StateData<GameData>) {
        self.level_number += 1
    }

    fn update(&mut self, _data: StateData<GameData>) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        if let Some(level) = self.level_path() {
            Trans::Push(Box::new(LevelLoaderState::for_level(level)))
        } else {
            Trans::Quit
        }
    }
}
