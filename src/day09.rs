use std::{
    collections::{BTreeMap, BTreeSet},
    iter::repeat,
};

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
    sum
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Puzzle) -> usize {
    let blocks = input.0.iter().sum();
    let mut disk: Vec<Option<usize>> = vec![None; blocks];
    let mut file_index: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut free_index: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut idx = 0;
    for (i, v) in input.0.iter().enumerate() {
        if i % 2 == 0 {
            let slice = &mut disk[idx..idx + v];
            slice.fill(Some(i / 2));
            file_index.insert(i / 2, (idx, *v));
        } else {
            free_index.entry(*v).or_default().insert(idx);
        }
        idx += v;
    }

    for (i, (idx, count)) in file_index.into_iter().rev() {
        if let Some((&free_count, free_idx)) = free_index
            .iter_mut()
            // Large enough free blocks on the left of current index.
            .filter(|(&k, v)| k >= count && v.first().unwrap_or(&idx) < &idx)
            // The leftmost
            .min_by(|(k1, v1), (k2, v2)| (v1, k1).cmp(&(v2, k2)))
            // Pop it from the free blocks
            .and_then(|(k, v)| Some((k, v.pop_first()?)))
        {
            // Swap these
            let new = &mut disk[free_idx..free_idx + count];
            new.fill(Some(i));
            let old = &mut disk[idx..idx + count];
            old.fill(None);
            // Index remaining free space
            if free_count > count {
                free_index
                    .entry(free_count - count)
                    .or_default()
                    .insert(free_idx + count);
            }
        }
    }

    disk.into_iter()
        .enumerate()
        .map(|(i, v)| i * v.unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = "2333133121414131402";

    #[test_case(TEST => 1928)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST =>2858)]
    fn part2(input: &str) -> usize {
        solve_part2(&input_generator(input))
    }
}
