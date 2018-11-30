use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{CellCoordinate, PIXEL_PER_CASE, LEVEL_HEIGHT};

/// Sync Cell coordinates with Transform;
pub struct CellCoordinateSystem;

impl<'s> System<'s> for CellCoordinateSystem {
    type SystemData = (ReadStorage<'s, CellCoordinate>, WriteStorage<'s, Transform>);

    fn run(&mut self, (cells, mut transforms): Self::SystemData) {
        for (cell, transform) in (&cells, &mut transforms).join() {
            transform.translation.x = (cell.x as f32 + 0.5) * PIXEL_PER_CASE;
            transform.translation.y = ((LEVEL_HEIGHT - cell.y) as f32 - 0.5) * PIXEL_PER_CASE;
        }
    }
}
