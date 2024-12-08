use crate::position::Position;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Movement {
    pub x: isize,
    pub y: isize,
}

impl Movement {
    pub fn between(p1: Position, p2: Position) -> Option<Self> {
        Some(Self {
            x: isize::try_from(p2.x).ok()? - isize::try_from(p1.x).ok()?,
            y: isize::try_from(p2.y).ok()? - isize::try_from(p1.y).ok()?,
        })
    }
}

pub const UP: Movement = Movement { x: 0, y: -1 };
pub const RIGHT: Movement = Movement { x: 1, y: 0 };
pub const DOWN: Movement = Movement { x: 0, y: 1 };
pub const LEFT: Movement = Movement { x: -1, y: 0 };

pub const CLOCKWISE: [Movement; 4] = [UP, RIGHT, DOWN, LEFT];
