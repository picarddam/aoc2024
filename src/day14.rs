use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use utils::{movement::Movement, position::Position};

#[derive(Debug)]
pub struct Puzzle(Vec<(Position, Movement)>);

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Puzzle {
    puzzle(input).unwrap().1
}

fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    map(separated_list1(newline, robot), Puzzle)(input)
}

fn robot(input: &str) -> IResult<&str, (Position, Movement)> {
    separated_pair(position, tag(" "), movement)(input)
}

fn position(input: &str) -> IResult<&str, Position> {
    map(
        preceded(tag("p="), separated_pair(u64, tag(","), u64)),
        |(x, y)| Position {
            x: x as usize,
            y: y as usize,
        },
    )(input)
}

fn movement(input: &str) -> IResult<&str, Movement> {
    map(
        preceded(tag("v="), separated_pair(i64, tag(","), i64)),
        |(x, y)| Movement {
            x: x as isize,
            y: y as isize,
        },
    )(input)
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    solve_part1_dim(input, 100, 101, 103)
}

pub fn solve_part1_dim(input: &Puzzle, seconds: u64, width: usize, height: usize) -> usize {
    let mut robots = input.0.clone();
    let iwidth = width as isize;
    let iheight = height as isize;
    let isecs = seconds as isize;
    for robot in &mut robots {
        robot.0.x = (robot.0.x as isize + robot.1.x * isecs).rem_euclid(iwidth) as usize;
        robot.0.y = (robot.0.y as isize + robot.1.y * isecs).rem_euclid(iheight) as usize;
    }
    let q = check_quadrant(&robots, width, height);
    q.0 * q.1 * q.2 * q.3
}

#[derive(Debug, Default)]
pub struct Quadrant(usize, usize, usize, usize);

pub fn check_quadrant(robots: &[(Position, Movement)], width: usize, height: usize) -> Quadrant {
    let mut out = Quadrant::default();
    let w = width / 2;
    let h = height / 2;
    for robot in robots {
        match (robot.0.x.cmp(&w), robot.0.y.cmp(&h)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => out.0 += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => out.1 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => out.2 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => out.3 += 1,
            _ => (),
        }
    }
    out
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Puzzle) -> u64 {
    todo!("Use {input:?} to solve step 2.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test_case(TEST => 12)]
    fn part1(input: &str) -> usize {
        solve_part1_dim(&input_generator(input), 100, 11, 7)
    }

    // #[test_case(TEST => 80)]
    // fn part2(input: &str) -> usize {
    //     solve_part2(&input_generator(input))
    // }
}
