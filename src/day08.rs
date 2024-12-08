use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};
use utils::{movement::Movement, position::Position};

pub struct Puzzle {
    height: usize,
    width: usize,
    data: HashMap<char, Vec<Position>>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Puzzle {
    let mut height = 0;
    let mut width = 0;
    let mut data: HashMap<char, Vec<Position>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        height = max(height, y + 1);
        for (x, p) in line.chars().enumerate() {
            width = max(width, x + 1);
            if p != '.' {
                data.entry(p).or_default().push(Position { x, y })
            }
        }
    }
    Puzzle {
        height,
        width,
        data,
    }
}

fn antinode(antenna: Position, other: Position, puzzle: &Puzzle) -> Option<Position> {
    let m = Movement::between(antenna, other)?;
    other
        .checked_move(&m)
        .filter(|&Position { x, y }| x < puzzle.width && y < puzzle.height)
}

fn antinodes(antennas: &[Position], puzzle: &Puzzle) -> HashSet<Position> {
    let mut output = HashSet::new();
    for (i, &antenna) in antennas.iter().enumerate() {
        for &other in &antennas[i + 1..] {
            output.insert(antinode(antenna, other, puzzle));
            output.insert(antinode(other, antenna, puzzle));
        }
    }
    output.into_iter().flatten().collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    input
        .data
        .iter()
        .flat_map(|(_, v)| antinodes(v, input))
        .collect::<HashSet<_>>()
        .len()
}

// #[aoc(day8, part2)]
// pub fn solve_part2(input: &Puzzle) -> u64 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const SIMPLER: &str = r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
"#;

    const THREE: &str = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
"#;

    #[test_case(TEST => 14)]
    #[test_case(SIMPLER => 2)]
    #[test_case(THREE => 4)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    // #[test_case(TEST => 11387)]
    // fn part2(input: &str) -> u64 {
    //     solve_part2(&input_generator(input))
    // }
}
