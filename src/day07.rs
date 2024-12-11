use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::math::asm_div_rem;

type Puzzle = Vec<(u64, Vec<u64>)>;

fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    separated_list1(
        newline,
        separated_pair(u64, tag(": "), separated_list1(space1, u64)),
    )(input)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Puzzle {
    puzzle(input).unwrap().1
}

fn valid_equation(result: u64, numbers: &[u64]) -> bool {
    match numbers {
        [] => false,
        [last] => *last == result,
        [head @ .., tail] => {
            let (quotient, remainder) = unsafe {
                // SAFETY: input does not contain 0, so YOLO
                asm_div_rem(result, *tail)
            };
            remainder == 0 && valid_equation(quotient, head)
                || result > *tail && valid_equation(result - tail, head)
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Puzzle) -> u64 {
    input
        .par_iter()
        .filter(|&(k, v)| valid_equation(*k, v))
        .map(|(k, _v)| k)
        .sum()
}

fn valid_equation_concat(result: u64, numbers: &[u64]) -> bool {
    match numbers {
        [] => unreachable!(),
        [last] => *last == result,
        [head @ .., last] => {
            let (quotient, remainder) = unsafe {
                // SAFETY: input does not contain 0, so YOLO
                asm_div_rem(result, *last)
            };
            let split = 10u64.pow(last.ilog10() + 1);
            let (prefix, suffix) = unsafe {
                // SAFETY: m is a power of 10 greater or equal to 10^1
                asm_div_rem(result, split)
            };
            remainder == 0 && valid_equation_concat(quotient, head)
                || result > *last && valid_equation_concat(result - last, head)
                || suffix == *last && valid_equation_concat(prefix, head)
        }
    }
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Puzzle) -> u64 {
    input
        .par_iter()
        .filter(|&(k, v)| valid_equation_concat(*k, v))
        .map(|(k, _v)| k)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test_case(TEST => 3749)]
    fn part1(input: &str) -> u64 {
        solve_part1(&input_generator(input))
    }

    #[test_case("190: 10 19" => true)]
    #[test_case("3267: 81 40 27" => true)]
    #[test_case("83: 17 5" => false)]
    #[test_case("156: 15 6" => false)]
    #[test_case("7290: 6 8 6 15" => false)]
    #[test_case("161011: 16 10 13" => false)]
    #[test_case("192: 17 8 14" => false)]
    #[test_case("21037: 9 7 18 13" => false)]
    #[test_case("292: 11 6 16 20" => true)]
    fn part1_line(input: &str) -> bool {
        let puzzle = input_generator(input);
        puzzle
            .first()
            .map(|(k, v)| valid_equation(*k, v))
            .unwrap_or_default()
    }

    #[test]
    fn part1_solve() {
        let input = include_str!("../input/2024/day7.txt");
        assert_eq!(part1(input), 2941973819040)
    }

    #[test_case(TEST => 11387)]
    fn part2(input: &str) -> u64 {
        solve_part2(&input_generator(input))
    }

    #[test_case("190: 10 19" => true)]
    #[test_case("3267: 81 40 27" => true)]
    #[test_case("83: 17 5" => false)]
    #[test_case("156: 15 6" => true)]
    #[test_case("7290: 6 8 6 15" => true)]
    #[test_case("161011: 16 10 13" => false)]
    #[test_case("192: 17 8 14" => true)]
    #[test_case("21037: 9 7 18 13" => false)]
    #[test_case("292: 11 6 16 20" => true)]
    fn part2_line(input: &str) -> bool {
        let puzzle = input_generator(input);
        puzzle
            .first()
            .map(|(k, v)| valid_equation_concat(*k, v))
            .unwrap_or_default()
    }
}
