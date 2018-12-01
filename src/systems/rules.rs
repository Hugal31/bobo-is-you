use amethyst::ecs::prelude::{
    InsertedFlag, Join, ModifiedFlag, ReadStorage, ReaderId, RemovedFlag, Resources, System, Write,
    WriteStorage,
};

use crate::components::*;

#[derive(Default)]
pub struct RulesUpdateSystem {
    inserted_id: Option<ReaderId<InsertedFlag>>,
    modified_id: Option<ReaderId<ModifiedFlag>>,
    removed_id: Option<ReaderId<RemovedFlag>>,
}

impl RulesUpdateSystem {
    fn try_resolve(
        rules: &mut Rules,
        name: Named,
        cell: CellCoordinate,
        insts: &ReadStorage<Instruction>,
        cells: &ReadStorage<CellCoordinate>,
    ) {
        // Try down
        if let Some(down) = cell.try_down(LEVEL_HEIGHT) {
            if let Some(downdown) = down.try_down(LEVEL_HEIGHT) {
                Self::try_resolve_for_cells(rules, name, down, downdown, (insts, cells))
            }
        }

        // Try right
        if let Some(right) = cell.try_right(LEVEL_WIDTH) {
            if let Some(rightright) = right.try_right(LEVEL_WIDTH) {
                Self::try_resolve_for_cells(rules, name, right, rightright, (insts, cells))
            }
        }
    }

    fn try_resolve_for_cells(
        rules: &mut Rules,
        name: Named,
        is_cell: CellCoordinate,
        cap_cell: CellCoordinate,
        (insts, cells): (&ReadStorage<Instruction>, &ReadStorage<CellCoordinate>),
    ) {
        if (cells, insts).join().any(|ci| match ci {
            (cell, Instruction::Is) if *cell == is_cell => true,
            _ => false,
        }) {
            if let Some(cap) = (cells, insts).join().find_map(|ci| match ci {
                (cell, Instruction::Cap(c)) if *cell == cap_cell => Some(c),
                _ => None,
            }) {
                *rules.caps_mut_for(name) = rules.caps_for(name) | *cap;
            }
        }
    }
}

impl<'s> System<'s> for RulesUpdateSystem {
    type SystemData = (
        Write<'s, Rules>,
        ReadStorage<'s, Instruction>,
        ReadStorage<'s, CellCoordinate>,
    );

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::SystemData;
        Self::SystemData::setup(res);

        let mut cells = WriteStorage::<CellCoordinate>::fetch(res);
        self.inserted_id = Some(cells.inserted_mut().register_reader());
        self.modified_id = Some(cells.modified_mut().register_reader());
        self.removed_id = Some(cells.removed_mut().register_reader());
    }

    fn run(&mut self, (mut rules, insts, cells): Self::SystemData) {
        // FIXME: Is there a better way than this?
        let modified = cells
            .modified()
            .read(self.modified_id.as_mut().expect("setup was not called"))
            .len() != 0;
        let inserted = cells
            .inserted()
            .read(self.inserted_id.as_mut().expect("setup was not called"))
            .len() != 0;
        let removed = cells
            .removed()
            .read(self.removed_id.as_mut().expect("setup was not called"))
            .len() != 0;

        if !modified && !inserted && !removed {
            return;
        }

        rules.reset();

        (&insts, &cells)
            .join()
            .filter_map(|(i, c)| match i {
                Instruction::Name(n) => Some((n, c)),
                _ => None,
            })
            .for_each(|(&name, &cell)| Self::try_resolve(&mut rules, name, cell, &insts, &cells));
    }
}
