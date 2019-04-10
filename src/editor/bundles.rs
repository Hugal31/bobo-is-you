use amethyst::core::bundle::*;
use amethyst::ecs::DispatcherBuilder;

use crate::editor::systems::cursor::CursorSystem;
use crate::systems::CellCoordinateSystem;

pub struct EditorBundle;

impl SystemBundle<'_, '_> for EditorBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<()> {
        dispatcher.add(CursorSystem::default(), "cursor", &[]);

        dispatcher.add(
            CellCoordinateSystem::default(),
            "cell_coordinate",
            &["cursor"],
        );

        Ok(())
    }
}
