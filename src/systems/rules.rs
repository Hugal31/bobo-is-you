use amethyst::ecs::prelude::{
    ComponentEvent, Join, ReadStorage, ReaderId, Resources, System, Write, WriteStorage,
};

use crate::components::*;

#[derive(Default)]
pub struct RulesUpdateSystem {
    cells_events_id: Option<ReaderId<ComponentEvent>>,
}

impl RulesUpdateSystem {
    fn try_resolve(
        rules: &mut Rules,
        name: Named,
        cell: CellCoordinate,
        (insts, cells, names): (
            &ReadStorage<Instruction>,
            &ReadStorage<CellCoordinate>,
            &mut WriteStorage<Named>,
        ),
    ) {
        // Try down
        if let Some(down) = cell.try_down(LEVEL_HEIGHT) {
            if let Some(downdown) = down.try_down(LEVEL_HEIGHT) {
                Self::try_resolve_for_cells(rules, name, down, downdown, (insts, cells, names))
            }
        }

        // Try right
        if let Some(right) = cell.try_right(LEVEL_WIDTH) {
            if let Some(rightright) = right.try_right(LEVEL_WIDTH) {
                Self::try_resolve_for_cells(rules, name, right, rightright, (insts, cells, names))
            }
        }
    }

    fn try_resolve_for_cells(
        rules: &mut Rules,
        name: Named,
        is_cell: CellCoordinate,
        cap_cell: CellCoordinate,
        (insts, cells, names): (
            &ReadStorage<Instruction>,
            &ReadStorage<CellCoordinate>,
            &mut WriteStorage<Named>,
        ),
    ) {
        // Search for the Is
        if (cells, insts).join().any(|ci| match ci {
            (cell, Instruction::Is) if *cell == is_cell => true,
            _ => false,
        }) {
            // Search for a Cap
            if let Some(cap) = (cells, insts).join().find_map(|ci| match ci {
                (cell, Instruction::Cap(c)) if *cell == cap_cell => Some(c),
                _ => None,
            }) {
                *rules.caps_mut_for(name) = rules.caps_for(name) | *cap;

            // Search for a name
            } else if let Some(other_name) = (cells, insts).join().find_map(|ci| match ci {
                (cell, Instruction::Name(n)) if *cell == cap_cell => Some(n),
                _ => None,
            }) {
                for name_to_transform in (names).join().filter(|&&mut n| n == name) {
                    debug!("Transform a {:?} into {:?}", name_to_transform, other_name);
                    *name_to_transform = *other_name;
                }
            }
        }
    }
}

impl<'s> System<'s> for RulesUpdateSystem {
    type SystemData = (
        Write<'s, Rules>,
        ReadStorage<'s, Instruction>,
        ReadStorage<'s, CellCoordinate>,
        WriteStorage<'s, Named>,
    );

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::SystemData;
        Self::SystemData::setup(res);

        let mut cells = WriteStorage::<CellCoordinate>::fetch(res);
        self.cells_events_id = Some(cells.register_reader());
    }

    fn run(&mut self, (mut rules, insts, cells, mut names): Self::SystemData) {
        let modified = cells
            .channel()
            .read(self.cells_events_id.as_mut().expect("setup was not called"))
            .next()
            .is_some();

        if !modified {
            return;
        }

        rules.reset();

        (&insts, &cells)
            .join()
            .filter_map(|(i, c)| match i {
                Instruction::Name(n) => Some((n, c)),
                _ => None,
            })
            .for_each(|(&name, &cell)| {
                Self::try_resolve(&mut rules, name, cell, (&insts, &cells, &mut names))
            });
    }
}
