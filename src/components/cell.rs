use amethyst::ecs::{storage::VecStorage, Component};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CellCoordinate {
    pub x: u32,
    pub y: u32,
}

impl CellCoordinate {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32) -> CellCoordinate {
        CellCoordinate { x, y }
    }

    pub fn try_up(self, min_y: u32) -> Option<CellCoordinate> {
        if self.y > min_y {
            Some(CellCoordinate {
                y: self.y - 1,
                ..self
            })
        } else {
            None
        }
    }

    pub fn try_down(self, max_y: u32) -> Option<CellCoordinate> {
        if self.y < max_y {
            Some(CellCoordinate {
                y: self.y + 1,
                ..self
            })
        } else {
            None
        }
    }

    pub fn try_right(self, max_x: u32) -> Option<CellCoordinate> {
        if self.x < max_x {
            Some(CellCoordinate {
                x: self.x + 1,
                ..self
            })
        } else {
            None
        }
    }

    pub fn try_left(self, min_x: u32) -> Option<CellCoordinate> {
        if self.x > min_x {
            Some(CellCoordinate {
                x: self.x - 1,
                ..self
            })
        } else {
            None
        }
    }
}

impl Component for CellCoordinate {
    type Storage = VecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_dirs() {
        let zero = CellCoordinate::new(1, 1);
        assert_eq!(None, zero.try_up(1));
        assert_eq!(None, zero.try_left(1));
        assert_eq!(Some(CellCoordinate::new(1, 0)), zero.try_up(0));
        assert_eq!(Some(CellCoordinate::new(0, 1)), zero.try_left(0));
        assert_eq!(Some(CellCoordinate::new(1, 2)), zero.try_down(9));
        assert_eq!(Some(CellCoordinate::new(2, 1)), zero.try_right(4));

        let max = CellCoordinate::new(5, 10);
        assert_eq!(Some(CellCoordinate::new(5, 9)), max.try_up(0));
        assert_eq!(Some(CellCoordinate::new(4, 10)), max.try_left(0));
        assert_eq!(Some(CellCoordinate::new(5, 11)), max.try_down(11));
        assert_eq!(Some(CellCoordinate::new(6, 10)), max.try_right(6));
        assert_eq!(None, max.try_down(10));
        assert_eq!(None, max.try_right(5));
    }
}
