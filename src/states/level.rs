use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::winit::VirtualKeyCode;

use super::LevelLoaderState;
use crate::components::*;
use crate::events::*;

pub struct LevelState {
    level_entity: Entity,
    level_name: String,
}

impl LevelState {
    pub fn new<S>(level_entity: Entity, level_name: S) -> LevelState
    where
        S: Into<String>,
    {
        LevelState {
            level_entity,
            level_name: level_name.into(),
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, BoboStateEvent> for LevelState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        camera::initialise_camera(self.level_entity, world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world
            .delete_entity(self.level_entity)
            .expect("Level entity was deleted");
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: BoboStateEvent,
    ) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        match &event {
            BoboStateEvent::Window(e)
                if is_close_requested(e) || is_key_down(&e, VirtualKeyCode::Escape) =>
            {
                Trans::Quit
            }
            BoboStateEvent::Window(e) if is_key_down(e, VirtualKeyCode::R) => Trans::Switch(
                Box::new(LevelLoaderState::for_level(self.level_name.as_ref())),
            ),
            BoboStateEvent::Game(GameEvent::Win) => Trans::Pop,
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        data.data.update(data.world);
        Trans::None
    }
}
