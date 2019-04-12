use amethyst::assets::PrefabLoaderSystem;
use amethyst::core::bundle::*;
use amethyst::ecs::DispatcherBuilder;

use crate::prefabs::{EntityPrefabData, LevelPrefabData};
use crate::systems::*;

pub struct BoboIsYouBundle;

impl SystemBundle<'_, '_> for BoboIsYouBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<()> {
        dispatcher.add(MoveActionSystem::default(), "move_action", &[]);
        dispatcher.add(
            CellCoordinateSystem::default(),
            "cell_coordinates",
            &["move_action"],
        );
        dispatcher.add(
            RulesUpdateSystem::default(),
            "rules_update",
            &["move_action"],
        );
        dispatcher.add(
            SyncNameAndSpriteSystem::default(),
            "sync_name_sprite",
            &["rules_update"],
        );
        dispatcher.add(WinSystem::default(), "win_system", &["move_action"]);
        dispatcher.add(PrefabLoaderSystem::<LevelPrefabData>::default(), "", &[]);
        dispatcher.add(PrefabLoaderSystem::<EntityPrefabData>::default(), "", &[]);

        Ok(())
    }
}
