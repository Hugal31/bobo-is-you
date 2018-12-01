use amethyst::assets::PrefabLoaderSystem;
use amethyst::core::bundle::*;
use amethyst::ecs::DispatcherBuilder;

use crate::prefabs::LevelPrefabData;
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
        dispatcher.add(PrefabLoaderSystem::<LevelPrefabData>::default(), "", &[]);

        Ok(())
    }
}
