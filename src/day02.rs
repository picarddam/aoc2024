use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    IResult,
};

fn input_line(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

fn input_puzzle(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(newline, input_line)(input)
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u64>> {
    input_puzzle(input).expect("failed to parse input").1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<u64>]) -> u64 {
    input.iter().filter(|l| is_safe(l)).count() as u64
}

fn is_safe(line: &[u64]) -> bool {
    problems(line) == 0
}

fn problems(line: &[u64]) -> u64 {
    let mut inc = 0;
    let mut dec = 0;
    let mut outrange = 0;
    for e in line.windows(2) {
        let l = e[0];
        let r = e[1];
        let diff = l.abs_diff(r);
        match l.cmp(&r) {
            Ordering::Less => inc += 1,
            Ordering::Equal => (),
            Ordering::Greater => dec += 1,
        };
        if !(1 <= diff && diff <= 3) {
            outrange += 1
        }
    }
    inc.min(dec) + outrange
}

fn is_safe_dampened(input: &[u64]) -> bool {
    (0..input.len()).any(|v| is_safe_without(input, v))
}

fn is_safe_without(input: &[u64], idx: usize) -> bool {
    let shortened: Vec<u64> = input
        .iter()
        .enumerate()
        .filter_map(|(i, e)| if i == idx { None } else { Some(*e) })
        .collect();
    is_safe(&shortened)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<u64>]) -> u64 {
    input.iter().filter(|line| is_safe_dampened(line)).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test_case(TEST => 2)]
    fn test_part1(input: &str) -> u64 {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST => 4)]
    fn test_part2(input: &str) -> u64 {
        solve_part2(&input_generator(input))
    }
}