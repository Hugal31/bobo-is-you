use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::InputEvent;
use amethyst::renderer::{SpriteRender, Transparent};
use amethyst::shred::DynamicSystemData;
use amethyst::shrev::EventChannel;

use crate::assets::GameAssets;
use crate::components::{CellCoordinate, Instruction, Named, LEVEL_BOUNDS};
use crate::direction::Direction;
use crate::editor::{components::Cursor, inputs::EditorInputAction};

/// Manage the cursor movement and actions
#[derive(Default)]
pub struct CursorSystem {
    action_reader: Option<ReaderId<InputEvent<EditorInputAction>>>,
}

impl CursorSystem {
    fn move_cursor(coord: &mut CellCoordinate, dir: Direction) {
        if let Some(new_coord) = coord.try_moved(dir, &LEVEL_BOUNDS) {
            *coord = new_coord;
        }
    }

    /// Filter InputEvent::ActionPressed -> EditorInputAction
    fn filter_map_editor_action_pressed(
        event: &InputEvent<EditorInputAction>,
    ) -> Option<&EditorInputAction> {
        match event {
            InputEvent::ActionPressed(e) => Some(e),
            _ => None,
        }
    }
}

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, EventChannel<InputEvent<EditorInputAction>>>,
        ReadExpect<'s, GameAssets>,
        ReadStorage<'s, Cursor>,
        WriteStorage<'s, CellCoordinate>,
        WriteStorage<'s, Instruction>,
        WriteStorage<'s, Named>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Transparent>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (entities, actions, assets, cursors, mut coords, mut _insts, mut named, mut transf, mut transp, mut sprite): Self::SystemData) {
        actions
            .read(self.action_reader.as_mut().expect("setup was not called"))
            .filter_map(Self::filter_map_editor_action_pressed)
            .for_each(|ei| {
                match ei {
                    EditorInputAction::Move(dir) => {
                        for (_, coord) in (&cursors, &mut coords).join() {
                            Self::move_cursor(coord, *dir);
                        }
                    },
                    EditorInputAction::Replace => {
                        let cursor_coord = (&cursors, &coords).join()
                            .map(|(_, coords)| coords)
                            .next()
                            .expect("There should be a cursor");
                        let entity = entities.create();
                        coords.insert(entity, *cursor_coord).unwrap();
                        named.insert(entity, Named::Bobo).unwrap();
                        transf.insert(entity, Transform::default());
                        transp.insert(entity, Transparent);
                        sprite.insert(entity, assets.entity_sprite(0)).unwrap();
                        debug!("Added new entity");
                    },
                    _ => (),
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), res);

        self.action_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<EditorInputAction>>>()
                .register_reader(),
        );
    }
}
