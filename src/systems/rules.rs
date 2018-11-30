use amethyst::ecs::{Join, ReadStorage, System, Write};

use crate::components::*;

pub struct RulesUpdateSystem;

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

    fn run(&mut self, (mut rules, insts, cells): Self::SystemData) {
        rules.reset();

        for (inst, cell) in (&insts, &cells).join() {
            if let Instruction::Name(name) = inst {
                Self::try_resolve(&mut rules, *name, *cell, &insts, &cells);
            }
        }
    }
}
