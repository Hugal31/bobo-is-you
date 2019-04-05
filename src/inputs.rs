use serde::{Deserialize, Serialize};

use crate::direction::Direction;

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum InputAction {
    Move(Direction)
}
