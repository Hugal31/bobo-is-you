use amethyst::ecs::prelude::{
    ComponentEvent, Join, Read, ReadStorage, ReaderId, Resources, System, Write, WriteStorage,
};
use amethyst::shrev::EventChannel;

use crate::components::*;
use crate::events::GameEvent;

#[derive(Default)]
pub struct WinSystem {
    cells_event_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for WinSystem {
    type SystemData = (
        Read<'a, Rules>,
        ReadStorage<'a, CellCoordinate>,
        ReadStorage<'a, Named>,
        Write<'a, EventChannel<GameEvent>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::SystemData;
        Self::SystemData::setup(res);

        let mut cells = WriteStorage::<CellCoordinate>::fetch(res);
        self.cells_event_id = Some(cells.register_reader());
    }

    fn run(&mut self, (rules, cells, names, mut game_events): Self::SystemData) {
        // FIXME: Is there a better way than this?
        let modified = cells
            .channel()
            .read(self.cells_event_id.as_mut().expect("setup was not called"))
            .filter(|event| match event {
                ComponentEvent::Modified(_id) => true,
                _ => false,
            })
            .next()
            .is_some();

        if !modified {
            return;
        }

        // Iter on all "you"
        for (&cell, _) in (&cells, &names)
            .join()
            .filter(|(_, &name)| rules.caps_for(name).is_you)
        {
            // If there is a "win" at the same cell, write a win event.
            if (&cells, &names).join().any(|(&other_cell, &other_name)| {
                rules.caps_for(other_name).is_win && cell == other_cell
            }) {
                game_events.single_write(GameEvent::Win);
                return;
            }
        }
    }
}
