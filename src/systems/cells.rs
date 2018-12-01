use amethyst::core::Transform;
use amethyst::ecs::prelude::{
    BitSet, InsertedFlag, Join, ModifiedFlag, ReadStorage, ReaderId, Resources, System,
    WriteStorage,
};

use crate::components::{CellCoordinate, LEVEL_HEIGHT, PIXEL_PER_CASE};

/// Sync Cell coordinates with Transform;
#[derive(Default)]
pub struct CellCoordinateSystem {
    inserted_id: Option<ReaderId<InsertedFlag>>,
    modified_id: Option<ReaderId<ModifiedFlag>>,

    modified: BitSet,
}

impl<'s> System<'s> for CellCoordinateSystem {
    type SystemData = (ReadStorage<'s, CellCoordinate>, WriteStorage<'s, Transform>);

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::prelude::SystemData;
        Self::SystemData::setup(res);

        let mut cells = WriteStorage::<CellCoordinate>::fetch(res);
        self.inserted_id = Some(cells.track_inserted());
        self.modified_id = Some(cells.track_modified());
    }

    fn run(&mut self, (cells, mut transforms): Self::SystemData) {
        self.modified.clear();

        cells.populate_modified(
            self.modified_id.as_mut().expect("setup was not called"),
            &mut self.modified,
        );
        cells.populate_inserted(
            self.inserted_id.as_mut().expect("setup was not called"),
            &mut self.modified,
        );

        for (cell, transform, _) in (&cells, &mut transforms, &self.modified).join() {
            transform.translation.x = (cell.x as f32 + 0.5) * PIXEL_PER_CASE;
            transform.translation.y = ((LEVEL_HEIGHT - cell.y) as f32 - 0.5) * PIXEL_PER_CASE;
        }
    }
}
