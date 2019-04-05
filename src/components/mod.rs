mod cell;
mod rules;

pub use self::cell::*;
pub use self::rules::*;

pub const PIXEL_PER_CASE: f32 = 32.0;
pub const LEVEL_WIDTH: u32 = 10;
pub const LEVEL_HEIGHT: u32 = 10;

/// Limits of a level
/// Limits are inclusive.
#[derive(Clone, Debug)]
pub struct Bounds {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32
}

impl Bounds {
    pub const fn with_size(width: u32, height: u32) -> Self {
        Bounds {
            min_x: 0,
            min_y: 0,
            max_x: width - 1,
            max_y: height - 1,
        }
    }
}

pub static LEVEL_BOUNDS: Bounds = Bounds::with_size(LEVEL_WIDTH, LEVEL_HEIGHT);
