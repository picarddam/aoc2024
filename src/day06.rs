use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use utils::grid::Grid;
use utils::movement::{Movement, CLOCKWISE, DOWN, LEFT, RIGHT, UP};
use utils::position::Position;

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
    get_visited(input)
        .into_iter()
        .map(|(p, _)| p)
        .collect::<HashSet<_>>()
        .len() as u64
}

fn get_visited(input: &Puzzle) -> Vec<(Position, Movement)> {
    let mut position = input
        .positions()
        .filter(|&(_, t)| *t == Tile::Guard)
        .map(|(p, _)| p)
        .next()
        .expect("Failed to find guard");
    let mut visited = Vec::new();
    let mut moves = CLOCKWISE.iter().cycle();
    let mut movement = moves.next().unwrap();
    visited.push((position, *movement));
    while let Some(next_pos) = input.checked_move(position, *movement) {
        let next_tile = input[&next_pos];
        match next_tile {
            Tile::Wall => {
                movement = moves.next().unwrap();
            }
            _ => {
                position = position + movement;
            }
        }
        visited.push((position, *movement));
    }
    visited
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Puzzle) -> u64 {
    let mut counter = 0;
    let mut visited: HashSet<Position> = HashSet::new();
    let mut curr_pos = input
        .positions()
        .filter(|&(_, t)| *t == Tile::Guard)
        .map(|(p, _)| p)
        .next()
        .expect("Failed to find guard");
    let mut curr_move = UP;
    while let Some(pos) = input.checked_move(curr_pos, curr_move) {
        visited.insert(curr_pos);
        if input[&pos] == Tile::Wall {
            curr_move = rotate(curr_move);
        } else {
            if !visited.contains(&pos) && is_loop(input, curr_pos, curr_move) {
                counter += 1;
            }
            curr_pos = pos;
        }
    }
    counter
}

fn rotate(m: Movement) -> Movement {
    if m == UP {
        return RIGHT;
    }
    if m == RIGHT {
        return DOWN;
    }
    if m == DOWN {
        return LEFT;
    }
    if m == LEFT {
        return UP;
    }
    unreachable!()
}

fn is_loop(grid: &Grid<Tile>, init: Position, movement: Movement) -> bool {
    let block = match grid
        .checked_move(init, movement)
        .filter(|p| grid[p] == Tile::Floor)
    {
        Some(pos) => pos,
        None => return false,
    };
    let mut visited: HashSet<(Position, Movement)> = HashSet::new();
    let mut curr_pos = init;
    let mut curr_move = movement;
    while let Some(pos) = grid.checked_move(curr_pos, curr_move) {
        if pos == block || grid[&pos] == Tile::Wall {
            if visited.contains(&(pos, curr_move)) {
                return true;
            }
            visited.insert((pos, curr_move));
            curr_move = rotate(curr_move);
        } else {
            curr_pos = pos;
        }
    }
    false
}

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

    #[test_case(TEST => 6)]
    fn part2(input: &str) -> u64 {
        solve_part2(&input_generator(input))
    }
}
