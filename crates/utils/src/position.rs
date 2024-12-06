use std::ops::Add;

use crate::movement::Movement;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn checked_move(&self, movement: &Movement) -> Option<Self> {
        Some(Position {
            x: self.x.checked_add_signed(movement.x)?,
            y: self.y.checked_add_signed(movement.y)?,
        })
    }
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
