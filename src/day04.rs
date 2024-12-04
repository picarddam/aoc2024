use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{Array2, ArrayView1, ArrayView2};

type Puzzle = Array2<u8>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Puzzle {
    let bfr: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<u8>>())
        .collect::<Vec<_>>();
    Array2::from_shape_vec(
        (bfr.len(), bfr[0].len()),
        bfr.iter().flatten().copied().collect(),
    )
    .unwrap()
}

fn bfr_filter(bfr: &ArrayView1<u8>) -> bool {
    const XMAS: [u8; 4] = [88, 77, 65, 83];
    let xmas = XMAS.iter().zip(bfr.iter()).all(|(a, b)| a == b);
    let samx = XMAS.iter().rev().zip(bfr.iter()).all(|(a, b)| a == b);
    xmas || samx
}

fn count_lines(grid: &ArrayView2<u8>) -> usize {
    grid.windows((1, 4))
        .into_iter()
        .map(|arr| bfr_filter(&arr.flatten().view()))
        .filter(|x| *x)
        .count()
}

fn count_columns(grid: &ArrayView2<u8>) -> usize {
    grid.windows((4, 1))
        .into_iter()
        .map(|arr| bfr_filter(&arr.flatten().view()))
        .filter(|x| *x)
        .count()
}

fn count_diagonals(grid: &ArrayView2<u8>) -> usize {
    grid.windows((4, 4))
        .into_iter()
        .map(|grid| {
            let mut bfr = [0u8; 4];
            for i in 0..bfr.len() {
                bfr[i] = grid[[i, i]];
            }
            let diag1 = bfr == "XMAS".as_bytes() || bfr == "SAMX".as_bytes();
            for i in 0..bfr.len() {
                bfr[i] = grid[[i, bfr.len() - 1 - i]];
            }
            let diag2 = bfr == "XMAS".as_bytes() || bfr == "SAMX".as_bytes();
            diag1 as usize + diag2 as usize
        })
        .sum()
}

fn check_xmas_grid(grid: &ArrayView2<u8>) -> usize {
    count_lines(grid) + count_columns(grid) + count_diagonals(grid)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    check_xmas_grid(&input.view())
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Puzzle) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    const TEST4X4: &str = r#"XMAS
XMAS
XMAS
XMAS"#;
    const TEST7X4: &str = r#"XMASAMX
XMASAMX
XMASAMX
XMASAMX"#;
    const TEST5X4: &str = r#"XMAS
XMAS
XMAS
XMAS
XMAS"#;

    const TESTV4X4: &str = r#"XSXS
MAMA
AMAM
SXSX"#;
    const TESTV5X4: &str = r#"XSXSX
MAMAM
AMAMA
SXSXS"#;
    const TESTD4X4: &str = r#"X...
.M..
..A.
...S"#;

    const TESTR4X4: &str = r#"...S
..A.
.M..
X..."#;
    #[test_case(TEST => 18)]
    #[test_case(TEST4X4 => 6)]
    #[test_case(TEST7X4 => 12)]
    #[test_case(TEST5X4 => 9)]
    fn part1(input: &str) -> usize {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST4X4 => 4)]
    #[test_case(TEST5X4 => 5)]
    fn part1_lines(input: &str) -> usize {
        let grid = input_generator(input);
        count_lines(&grid.view())
    }

    #[test_case(TEST4X4 => 0)]
    #[test_case(TEST5X4 => 0)]
    #[test_case(TESTV4X4 => 4)]
    #[test_case(TESTV5X4 => 5)]
    fn part1_columns(input: &str) -> usize {
        let grid = input_generator(input);
        count_columns(&grid.view())
    }

    #[test_case(TEST4X4 => 2)]
    #[test_case(TEST5X4 => 4)]
    #[test_case(TESTV4X4 => 0)]
    #[test_case(TESTV5X4 => 0)]
    #[test_case(TESTD4X4 => 1)]
    #[test_case(TESTR4X4 => 1)]
    fn part1_diagonals(input: &str) -> usize {
        let grid = input_generator(input);
        count_diagonals(&grid.view())
    }

    // #[test_case(TEST => 48)]
    // fn part2(input: &str) -> u64 {
    //     solve_part2(&input_generator(input))
    // }
}
