use amethyst::assets::{Completion, PrefabLoader, ProgressCounter, RonFormat};
use amethyst::ecs::Entity;
use amethyst::prelude::*;

use crate::events::BoboStateEvent;
use crate::prefabs::LevelPrefabData;

use super::LevelState;

pub struct LevelLoaderState {
    level_name: String,
    /// Progress tracker.
    progress: ProgressCounter,
    level_entity: Option<Entity>,
}

impl LevelLoaderState {
    pub fn for_level(level_name: impl Into<String>) -> LevelLoaderState {
        LevelLoaderState {
            level_name: level_name.into(),
            progress: ProgressCounter::new(),
            level_entity: None,
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, BoboStateEvent> for LevelLoaderState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        let prefab_handler = world.exec(|loader: PrefabLoader<LevelPrefabData>| {
            loader.load(self.level_name.clone(), RonFormat, (), &mut self.progress)
        });

        self.level_entity = Some(world.create_entity().with(prefab_handler).build());
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        data.data.update(data.world);

        match self.progress.complete() {
            Completion::Complete => {
                info!("Level {} loaded!", self.level_name);
                Trans::Switch(Box::new(LevelState::new(
                    self.level_entity.expect("on_start was not called"),
                )))
            }
            Completion::Failed => Trans::Quit,
            Completion::Loading => Trans::None,
        }
    }
}
