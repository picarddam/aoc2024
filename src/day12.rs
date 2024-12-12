use std::{collections::HashSet, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use utils::{
    grid::Grid,
    movement::{Movement, CLOCKWISE, DOWN, LEFT, RIGHT, UP},
    position::Position,
};

pub struct Puzzle(Grid<char>);

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle(Grid::from_vec(
            s.lines().map(|line| line.chars().collect()).collect(),
        )))
    }
}

impl Puzzle {
    fn count_fences(&self, p: &Position) -> usize {
        let mut fences = 0;
        let c = self.0[p];
        for m in CLOCKWISE.iter() {
            match self.0.checked_move(*p, *m) {
                Some(o) => fences += (c != self.0[&o]) as usize,
                None => fences += 1,
            }
        }
        fences
    }

    fn flood_fill(&self, p: &Position) -> HashSet<Position> {
        let mut stack = vec![*p];
        let c = self.0[p];
        let mut visited = HashSet::new();
        while let Some(p) = stack.pop() {
            visited.insert(p);
            for m in CLOCKWISE.iter() {
                if let Some(n) = self
                    .0
                    .checked_move(p, *m)
                    .filter(|n| self.0[n] == c && !visited.contains(n))
                {
                    stack.push(n);
                }
            }
        }
        visited
    }

    fn areas(&self) -> Vec<(char, HashSet<Position>)> {
        let mut areas = Vec::new();
        let mut visited: HashSet<Position> = HashSet::new();
        for (p, v) in self.0.positions() {
            if !visited.contains(&p) {
                let area = self.flood_fill(&p);
                visited.extend(&area);
                areas.push((*v, area));
            }
        }
        areas
    }

    fn is_corner(&self, point: &Position, d1: &Movement, d2: &Movement) -> bool {
        let v = self.0[point];
        let d3 = Movement {
            x: d1.x + d2.x,
            y: d1.y + d2.y,
        };
        let adj_up = self
            .0
            .checked_move(*point, *d1)
            .filter(|n| self.0[n] == v)
            .is_some();
        let adj_right = self
            .0
            .checked_move(*point, *d2)
            .filter(|n| self.0[n] == v)
            .is_some();
        let adj_diag = self
            .0
            .checked_move(*point, d3)
            .filter(|n| self.0[n] == v)
            .is_some();
        (!adj_diag && (adj_up == adj_right)) || (adj_diag && !adj_up && !adj_right)
    }

    fn count_sides(&self, poly: &HashSet<Position>) -> usize {
        let mut corners = 0;
        const CORNERS: [(Movement, Movement); 4] =
            [(UP, RIGHT), (RIGHT, DOWN), (DOWN, LEFT), (LEFT, UP)];
        for p in poly {
            for corner in CORNERS {
                if self.is_corner(p, &corner.0, &corner.1) {
                    corners += 1
                }
            }
        }
        corners
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Puzzle {
    Puzzle::from_str(input).expect("Failed to parse puzzle")
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    input
        .areas()
        .iter()
        .map(|(_, positions)| {
            positions.len()
                * positions
                    .iter()
                    .map(|position| input.count_fences(position))
                    .sum::<usize>()
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Puzzle) -> usize {
    input
        .areas()
        .iter()
        .map(|(_, positions)| positions.len() * input.count_sides(positions))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const SIMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    const INCLUSION: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const HARD: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const E: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const ALT: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test_case(SIMPLE => 140)]
    #[test_case(INCLUSION => 772)]
    #[test_case(HARD => 1930)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    #[test_case(SIMPLE => 80)]
    #[test_case(E => 236)]
    #[test_case(ALT => 368)]
    #[test_case(HARD => 1206)]
    fn part2(input: &str) -> usize {
        solve_part2(&input_generator(input))
    }
}
