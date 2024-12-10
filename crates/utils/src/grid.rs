use crate::position::Position;
use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    /// Create a grid from its dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::with_capacity(width * height),
        }
    }

    /// Create a grid from Vec of rows.
    pub fn from_vec(input: Vec<Vec<T>>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let data = input.into_iter().flatten().collect();
        Self {
            width,
            height,
            data,
        }
    }

    pub fn contains(&self, position: Position) -> bool {
        position.x < self.width && position.y < self.height
    }

    pub fn positions(&self) -> PosIter<'_, T> {
        PosIter::new(self)
    }
}

impl<T> Index<&Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.data[index.y * self.width + index.x]
    }
}

impl<T> IndexMut<&Position> for Grid<T> {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.data[index.y * self.width + index.x]
    }
}

pub struct PosIter<'a, T> {
    grid: &'a Grid<T>,
    current: usize,
}

impl<'a, T> PosIter<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, current: 0 }
    }
}

impl<'a, T> Iterator for PosIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.grid.data.len() {
            return None;
        }
        let position = Position {
            y: self.current / self.grid.height,
            x: self.current % self.grid.height,
        };
        let value = &self.grid.data[self.current];
        self.current += 1;
        Some((position, value))
    }
}
