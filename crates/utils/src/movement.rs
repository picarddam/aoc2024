#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Movement {
    pub x: isize,
    pub y: isize,
}

pub const UP: Movement = Movement { x: 0, y: -1 };
pub const RIGHT: Movement = Movement { x: 1, y: 0 };
pub const DOWN: Movement = Movement { x: 0, y: 1 };
pub const LEFT: Movement = Movement { x: -1, y: 0 };

pub const CLOCKWISE: [Movement; 4] = [UP, RIGHT, DOWN, LEFT];
