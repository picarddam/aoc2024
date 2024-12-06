use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use utils::grid::Grid;
use utils::movement::CLOCKWISE;

type Puzzle = Grid<Tile>;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Guard,
    Floor,
    Wall,
}

fn guard(input: &str) -> IResult<&str, Tile> {
    map(tag("^"), |_| Tile::Guard)(input)
}

fn floor(input: &str) -> IResult<&str, Tile> {
    map(tag("."), |_| Tile::Floor)(input)
}

fn wall(input: &str) -> IResult<&str, Tile> {
    map(tag("#"), |_| Tile::Wall)(input)
}

fn tile(input: &str) -> IResult<&str, Tile> {
    alt((guard, floor, wall))(input)
}

fn row(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(tile)(input)
}

fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    map(separated_list1(newline, row), Grid::from_vec)(input)
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Puzzle {
    puzzle(input).unwrap().1
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Puzzle) -> u64 {
    let mut position = input
        .positions()
        .filter(|&(_, t)| *t == Tile::Guard)
        .map(|(p, _)| p)
        .next()
        .expect("Failed to find guard");
    let mut visited = HashSet::new();
    visited.insert(position);
    let mut moves = CLOCKWISE.iter().cycle();
    let mut movement = moves.next().unwrap();
    while input.contains(position + movement) {
        let next_pos = position + movement;
        let next_tile = input[&next_pos];
        match next_tile {
            Tile::Wall => {
                movement = moves.next().unwrap();
            }
            _ => {
                position = position + movement;
                visited.insert(position);
            }
        }
    }
    visited.len() as u64
}

// #[aoc(day6, part2)]
// pub fn solve_part2(input: &Puzzle) -> u64 {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test_case(TEST => 41)]
    fn part1(input: &str) -> u64 {
        solve_part1(&input_generator(input))
    }

    // #[test_case(TEST => 123)]
    // fn part2(input: &str) -> u64 {
    //     solve_part2(&input_generator(input))
    // }
}
