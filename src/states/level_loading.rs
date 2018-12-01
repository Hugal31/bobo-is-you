use amethyst::assets::{PrefabLoader, ProgressCounter, RonFormat};
use amethyst::ecs::Entity;
use amethyst::prelude::*;

use crate::prefabs::LevelPrefabData;

use super::LevelState;

pub struct LevelLoaderState {
    level_name: String,
    /// Progress tracker.
    progress: ProgressCounter,
    level_entity: Option<Entity>,
}

impl LevelLoaderState {
    pub fn new(level_name: impl Into<String>) -> LevelLoaderState {
        LevelLoaderState {
            level_name: level_name.into(),
            progress: ProgressCounter::new(),
            level_entity: None,
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LevelLoaderState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        let prefab_handler = world.exec(|loader: PrefabLoader<LevelPrefabData>| {
            loader.load(self.level_name.clone(), RonFormat, (), ())
        });

        self.level_entity = Some(world.create_entity().with(prefab_handler).build());
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world);

        if self.progress.is_complete() {
            debug!("Level loading complete!");
            Trans::Switch(Box::new(LevelState::new(
                self.level_entity.expect("on_start was not called"),
            )))
        } else {
            Trans::None
        }
    }
}
