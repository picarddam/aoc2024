use std::ops::Add;

use crate::movement::Movement;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Add<Movement> for Position {
    type Output = Position;

    fn add(self, rhs: Movement) -> Self::Output {
        Position {
            x: self.x.checked_add_signed(rhs.x).unwrap(),
            y: self.y.checked_add_signed(rhs.y).unwrap(),
        }
    }
}

impl Add<&Movement> for Position {
    type Output = Position;

    fn add(self, rhs: &Movement) -> Self::Output {
        Position {
            x: self.x.checked_add_signed(rhs.x).unwrap(),
            y: self.y.checked_add_signed(rhs.y).unwrap(),
        }
    }
}
