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
    loop {
        visited.push((position, *movement));
        let next_pos = match position.checked_move(movement) {
            Some(pos) => pos,
            None => break,
        };
        if !input.contains(next_pos) {
            break;
        };
        let next_tile = input[&next_pos];
        match next_tile {
            Tile::Wall => {
                movement = moves.next().unwrap();
            }
            _ => {
                position = position + movement;
            }
        }
    }
    visited
}

fn is_loop(grid: &Grid<Tile>, init: Position, block: Position) -> bool {
    let mut curr_move = UP;
    let mut curr_pos = init;
    let mut visited: HashSet<(Position, Movement)> = HashSet::new();
    if !grid.contains(block) || grid[&block] == Tile::Wall {
        return false;
    }
    while !visited.contains(&(curr_pos, curr_move)) {
        let try_pos = match curr_pos.checked_move(&curr_move) {
            Some(pos) => pos,
            None => return false,
        };
        if !grid.contains(try_pos) {
            return false;
        }
        if grid[&try_pos] == Tile::Wall || try_pos == block {
            curr_move = rotate(curr_move);
        } else {
            visited.insert((curr_pos, curr_move));
            curr_pos = try_pos;
        }
    }
    true
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Puzzle) -> u64 {
    find_obstacles(input).len() as u64
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

fn find_obstacles(input: &Puzzle) -> HashSet<Position> {
    let path = get_visited(input);
    let init = path[0].0;
    path.iter()
        .filter(|&(p, m)| is_loop(input, init, *p + *m))
        .map(|&(p, m)| p + m)
        .filter(|p| *p != init)
        .collect::<HashSet<_>>()
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

    #[test_case(TEST, &[(6, 3), (7, 6), (7,7), (8,1), (8,3), (9,7)])]
    fn check_obstacle(input: &str, obstacles: &[(usize, usize)]) {
        let puzzle = input_generator(input);
        let obstacle_set: HashSet<Position> =
            obstacles.iter().map(|&(y, x)| Position { x, y }).collect();
        assert_eq!(
            find_obstacles(&puzzle)
                .difference(&obstacle_set)
                .copied()
                .collect::<Vec<_>>(),
            []
        )
    }

    #[test]
    fn check_not_starting() {
        let input = include_str!("../input/2024/day6.txt");
        let puzzle = input_generator(input);
        let guard = puzzle
            .positions()
            .filter(|&(_, t)| *t == Tile::Guard)
            .map(|(p, _)| p)
            .next()
            .expect("Failed to find guard");
        let potentials = find_obstacles(&puzzle);
        assert!(!potentials.contains(&guard));
        assert_eq!(potentials.len(), 2022);
    }
}
