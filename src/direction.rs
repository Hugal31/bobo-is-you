use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
pub static ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];
