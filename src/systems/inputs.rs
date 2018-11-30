use amethyst::ecs::{Entities, Entity, Join, Read, ReadStorage, Resources, System, WriteStorage};
use amethyst::input::InputEvent;
use amethyst::shred::DynamicSystemData;
use amethyst::shrev::{EventChannel, ReaderId};

use crate::components::*;
use crate::inputs::InputAction;

/// Sync Cell coordinates with Transform;
#[derive(Default)]
pub struct MoveActionSystem {
    action_reader: Option<ReaderId<InputEvent<InputAction>>>,
}

impl MoveActionSystem {
    fn next_cell(cell: CellCoordinate, action: InputAction) -> Option<CellCoordinate> {
        match action {
            InputAction::Up => cell.try_up(0),
            InputAction::Right => cell.try_right(LEVEL_WIDTH - 1),
            InputAction::Down => cell.try_down(LEVEL_HEIGHT - 1),
            InputAction::Left => cell.try_left(0),
        }
    }

    fn try_move_entity(
        entity: Entity,
        new_cell: CellCoordinate,
        rules: &Rules,
        (nameds, cells): (&ReadStorage<Named>, &mut WriteStorage<CellCoordinate>),
    ) {
        // Note: The &* is to duplicate the ref, join() move the values.
        // If someone has a better idea, I would like to know.
        if (&*nameds, &*cells)
            .join()
            .any(|(name, pos)| rules.caps_for(*name).is_stop && *pos == new_cell)
        {
            return;
        }

        cells.get_mut(entity).unwrap().x = new_cell.x;
        cells.get_mut(entity).unwrap().y = new_cell.y;
    }
}

impl<'s> System<'s> for MoveActionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'s>,
        Read<'s, Rules>,
        Read<'s, EventChannel<InputEvent<InputAction>>>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, CellCoordinate>,
    );

    fn setup(&mut self, res: &mut Resources) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), res);

        self.action_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<InputAction>>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (entities, rules, actions, names, mut cells): Self::SystemData) {
        for action in actions.read(self.action_reader.as_mut().expect("setup was not called")) {
            if let InputEvent::ActionPressed(a) = action {
                let to_move = (&entities, &names, &mut cells)
                    .join()
                    .filter(|(_, name, _)| rules.caps_for(**name).is_you)
                    .flat_map(|(e, _, cell)| {
                        MoveActionSystem::next_cell(*cell, *a).map(|cell| (e, cell))
                    })
                    .collect::<Vec<_>>();

                // TODO Sort to_move

                for (entity, new_pos) in &to_move {
                    MoveActionSystem::try_move_entity(
                        *entity,
                        *new_pos,
                        &rules,
                        (&names, &mut cells),
                    );
                }
            }
        }
    }
}
