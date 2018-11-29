use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum InputAction {
    Up,
    Right,
    Down,
    Left,
}
