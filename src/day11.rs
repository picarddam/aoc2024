use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Puzzle(Vec<u64>);

impl FromStr for Puzzle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_whitespace()
            .map(|token| token.parse::<u64>())
            .collect::<Result<_, _>>()?;
        Ok(Puzzle(stones))
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Puzzle {
    Puzzle::from_str(input).expect("Failed to parse puzzle")
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    let mut cache = HashMap::new();
    input
        .0
        .iter()
        .map(|&stone| stone_count(stone, 25, &mut cache) as usize)
        .sum()
}

fn stone_count(stone: u64, blink_left: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    match cache.get(&(stone, blink_left)) {
        Some(count) => *count,
        None => {
            let count = if blink_left == 0 {
                1
            } else if stone == 0 {
                stone_count(1, blink_left - 1, cache)
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let split = 10u64.pow((stone.ilog10() + 1) / 2);
                let left = stone_count(stone / split, blink_left - 1, cache);
                let right = stone_count(stone % split, blink_left - 1, cache);
                left + right
            } else {
                stone_count(stone * 2024, blink_left - 1, cache)
            };
            cache.insert((stone, blink_left), count);
            count
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Puzzle) -> usize {
    let mut cache = HashMap::new();
    input
        .0
        .iter()
        .map(|&stone| stone_count(stone, 75, &mut cache) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = "125 17";

    #[test_case(TEST => 55312)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST => 65601038650482)]
    fn part2(input: &str) -> usize {
        solve_part2(&input_generator(input))
    }
}
