use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use utils::{grid::Grid, movement::CLOCKWISE, position::Position};

type Puzzle = Grid<u32>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Puzzle {
    Grid::from_vec(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|chr| match chr {
                        '.' => Some(u32::MAX),
                        chr => chr.to_digit(10),
                    })
                    .collect()
            })
            .collect(),
    )
}

fn score_from(position: Position, elevation: u32, grid: &Grid<u32>) -> HashSet<Position> {
    if elevation == 9 {
        return HashSet::from([position]);
    }
    CLOCKWISE
        .iter()
        .flat_map(|m| grid.checked_move(position, *m))
        .filter(|n| grid[n] == elevation + 1)
        .fold(HashSet::new(), |mut h, n| {
            h.extend(score_from(n, grid[&n], grid));
            h
        })
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    let trailheads = input
        .positions()
        .filter(|(_, &h)| h == 0)
        .map(|(p, h)| (p, *h))
        .collect::<Vec<(Position, u32)>>();
    trailheads
        .iter()
        .map(|(p, h)| score_from(*p, *h, input).len())
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Puzzle) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = "0123
7654
8901";

    const TEST_FORK: &str = "9990999
9991999
9992999
6543456
7111117
8111118
9111119
";

    const TEST_LARGER: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    const TEST_MULTIPLE: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const TEST_TWO_HEADS: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";

    #[test_case(TEST => 1)]
    #[test_case(TEST_FORK => 2)]
    #[test_case(TEST_MULTIPLE => 4)]
    #[test_case(TEST_TWO_HEADS => 3)]
    #[test_case(TEST_LARGER => 36)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST_TWO_HEADS => vec![1, 2])]
    #[test_case(TEST_LARGER => vec![5, 6, 5, 3, 1, 3, 5, 3, 5])]
    fn part1_scores(input: &str) -> Vec<usize> {
        let puzzle = input_generator(input);
        let trailheads = puzzle.positions().filter(|(_, &h)| h == 0);
        let out: Vec<usize> = trailheads
            .map(|(p, &h)| score_from(p, h, &puzzle).len())
            .collect();
        out
    }

    #[test_case(TEST =>2858)]
    fn part2(input: &str) -> usize {
        solve_part2(&input_generator(input))
    }
}
