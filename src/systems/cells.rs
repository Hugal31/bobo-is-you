use amethyst::core::Transform;
use amethyst::ecs::prelude::{
    BitSet, ComponentEvent, Join, ReadStorage, ReaderId, Resources, System, WriteStorage,
};

use crate::components::{CellCoordinate, LEVEL_HEIGHT, PIXEL_PER_CASE};

/// Sync Cell coordinates with Transform;
#[derive(Default)]
pub struct CellCoordinateSystem {
    cells_events_id: Option<ReaderId<ComponentEvent>>,

    modified: BitSet,
}

impl<'s> System<'s> for CellCoordinateSystem {
    type SystemData = (ReadStorage<'s, CellCoordinate>, WriteStorage<'s, Transform>);

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::prelude::SystemData;
        Self::SystemData::setup(res);

        let mut cells = WriteStorage::<CellCoordinate>::fetch(res);
        self.cells_events_id = Some(cells.register_reader());
    }

    fn run(&mut self, (cells, mut transforms): Self::SystemData) {
        self.modified.clear();

        cells
            .channel()
            .read(self.cells_events_id.as_mut().expect("setup was not called"))
            .for_each(|event| match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.modified.add(*id);
                }
                ComponentEvent::Removed(_id) => (),
            });

        for (cell, transform, _) in (&cells, &mut transforms, &self.modified).join() {
            transform.set_x((cell.x as f32 + 0.5) * PIXEL_PER_CASE);
            transform.set_y(((LEVEL_HEIGHT - cell.y) as f32 - 0.5) * PIXEL_PER_CASE);
        }
    }
}
