use serde::{Deserialize, Serialize};

use crate::direction::Direction;

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum EditorInputAction {
    Move(Direction),
    Save,
    Replace, // TODO: Add what to replace
}
