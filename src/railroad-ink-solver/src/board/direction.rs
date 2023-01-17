use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

use Direction::{East, North, South, West};

impl Direction {
    #[must_use]
    pub fn inverse(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}
