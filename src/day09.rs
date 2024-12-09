use std::iter::repeat;

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Puzzle(Vec<usize>);

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Puzzle {
    Puzzle(
        input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|v| v as usize))
            .collect(),
    )
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    let mut file_indexes = input
        .0
        .chunks(2)
        .enumerate()
        .flat_map(|(i, c)| repeat(i).take(c[0]));
    let mut global_index = 0;
    let mut sum = 0;
    for (i, v) in input.0.iter().enumerate() {
        if i % 2 == 0 {
            sum += file_indexes
                .by_ref()
                .take(*v)
                .enumerate()
                .map(|(c, x)| x * (c + global_index))
                .sum::<usize>()
        } else {
            sum += file_indexes
                .by_ref()
                .rev()
                .take(*v)
                .enumerate()
                .map(|(c, x)| x * (c + global_index))
                .sum::<usize>()
        };
        global_index += v
    }
    println!();
    sum
}

// #[aoc(day9, part2)]
// pub fn solve_part2(input: &Puzzle) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = "2333133121414131402";

    #[test_case(TEST => 1928)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    // #[test_case(TEST => 34)]
    // fn part2(input: &str) -> usize {
    //     solve_part2(&input_generator(input))
    // }
}
