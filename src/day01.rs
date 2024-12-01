use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{multispace0, newline, u64},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn input_line(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(u64, multispace0, u64)(input)
}

fn input_puzzle(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list0(newline, input_line)(input)
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u64>, Vec<u64>) {
    let raw = input_puzzle(input).expect("failed to parse input").1;
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = raw.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let freqmap: HashMap<u64, u64> = input.1.iter().fold(HashMap::new(), |mut map, elem| {
        *map.entry(*elem).or_default() += 1;
        map
    });
    input
        .0
        .iter()
        .filter_map(|e| freqmap.get(e).map(|count| e * count))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test_case(TEST => 11)]
    fn test_day01_part1(input: &str) -> u64 {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST => 31)]
    fn test_day01_part2(input: &str) -> u64 {
        solve_part2(&input_generator(input))
    }
}
