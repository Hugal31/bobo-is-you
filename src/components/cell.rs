use amethyst::ecs::prelude::{Component, FlaggedStorage, VecStorage};
use serde::{Deserialize, Serialize};

use crate::components::Bounds;
use crate::direction::Direction;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CellCoordinate {
    pub x: u32,
    pub y: u32,
}

impl CellCoordinate {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32) -> CellCoordinate {
        CellCoordinate { x, y }
    }

    pub fn try_moved(self, direction: Direction, bounds: &Bounds) -> Option<Self> {
        match direction {
            Direction::North if self.y > bounds.min_y => Some(CellCoordinate {
                y: self.y - 1,
                ..self
            }),
            Direction::South if self.y < bounds.max_y => Some(CellCoordinate {
                y: self.y + 1,
                ..self
            }),
            Direction::East if self.x < bounds.max_x => Some(CellCoordinate {
                x: self.x + 1,
                ..self
            }),
            Direction::West if self.x > bounds.min_x => Some(CellCoordinate {
                x: self.x - 1,
                ..self
            }),
            _ => None,
        }
    }
}

impl Component for CellCoordinate {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_moved() {
        let bounds = Bounds {
            min_x: 1,
            min_y: 1,
            max_x: 5,
            max_y: 7,
        };

        let one = CellCoordinate::new(1, 1);
        assert_eq!(None, one.try_moved(Direction::North, &bounds));
        assert_eq!(None, one.try_moved(Direction::West, &bounds));
        assert_eq!(
            Some(CellCoordinate::new(2, 1)),
            one.try_moved(Direction::East, &bounds)
        );
        assert_eq!(
            Some(CellCoordinate::new(1, 2)),
            one.try_moved(Direction::South, &bounds)
        );
    }
}
