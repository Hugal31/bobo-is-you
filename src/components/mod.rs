mod cell;
mod rules;

pub use self::cell::*;
pub use self::rules::*;

pub const PIXEL_PER_CASE: f32 = 32.0;
pub const LEVEL_WIDTH: u32 = 10;
pub const LEVEL_HEIGHT: u32 = 10;
