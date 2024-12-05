use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, newline, u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{sequence::separated_pair, IResult};

pub struct Puzzle {
    ruleset: Vec<(u64, u64)>,
    print_queue: Vec<Vec<u64>>,
}

fn rule(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(u64, tag("|"), u64)(input)
}

fn ruleset(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(newline, rule)(input)
}

fn print(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), u64)(input)
}

fn print_queue(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(newline, print)(input)
}

fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    map(
        separated_pair(ruleset, multispace0, print_queue),
        |(r, p)| Puzzle {
            ruleset: r,
            print_queue: p,
        },
    )(input)
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Puzzle {
    puzzle(input).unwrap().1
}

fn rulemap(ruleset: &[(u64, u64)]) -> HashMap<u64, HashSet<u64>> {
    ruleset.iter().fold(HashMap::new(), |mut m, (k, v)| {
        m.entry(*k).or_default().insert(*v);
        m
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Puzzle) -> u64 {
    let rules = rulemap(&input.ruleset);
    input
        .print_queue
        .iter()
        .filter(|line| is_valid_line(line, &rules))
        .map(|line| line[line.len() / 2])
        .sum()
}

fn is_valid_line(line: &[u64], rules: &HashMap<u64, HashSet<u64>>) -> bool {
    line.iter().enumerate().all(|(i, v)| {
        let allowed = rules.get(v);
        let remainder = &line[i + 1..];
        match allowed {
            Some(allowed) => remainder.iter().all(|o| allowed.contains(o)),
            None => remainder.is_empty(),
        }
    })
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Puzzle) -> u64 {
    let rules = rulemap(&input.ruleset);
    let mut invalid: Vec<Vec<u64>> = input
        .print_queue
        .iter()
        .filter(|line| !is_valid_line(line, &rules))
        .cloned()
        .collect();
    invalid.iter_mut().for_each(|line| {
        line.sort_by(|a, b| match rules.get(a) {
            Some(allowed) => {
                if allowed.contains(b) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            None => Ordering::Less,
        })
    });
    invalid.iter().map(|l| l[l.len() / 2]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test_case(TEST => 143)]
    fn part1(input: &str) -> u64 {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST => 123)]
    fn part2(input: &str) -> u64 {
        solve_part2(&input_generator(input))
    }
}
