use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u64},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
    Noop,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(
            delimited(tag("mul("), separated_pair(u64, tag(","), u64), tag(")")),
            |(a, b)| Instruction::Mul(a, b),
        ),
        map(tag("do()"), |_| Instruction::Do),
        map(tag("don't()"), |_| Instruction::Dont),
        map(anychar, |_| Instruction::Noop),
    ))(input)
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    many1(instruction)(input).expect("failed to parse input").1
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Instruction]) -> u64 {
    input
        .iter()
        .fold((0, true), |(res, state), op| match op {
            Instruction::Mul(a, b) => (res + a * b, state),
            _ => (res, state),
        })
        .0
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Instruction]) -> u64 {
    input
        .iter()
        .fold((0, true), |(res, state), op| match op {
            Instruction::Mul(a, b) => (res + (if state { a * b } else { 0 }), state),
            Instruction::Noop => (res, state),
            Instruction::Do => (res, true),
            Instruction::Dont => (res, false),
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const TEST2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test_case(TEST => 161)]
    fn part1(input: &str) -> u64 {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST2 => 48)]
    fn part2(input: &str) -> u64 {
        solve_part2(&input_generator(input))
    }
}
