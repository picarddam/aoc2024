use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{array, Axis};
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, u32},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
pub struct V2 {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct Buttons {
    a: V2,
    b: V2,
}

#[derive(Debug)]
pub struct Machine {
    buttons: Buttons,
    prize: V2,
}

#[derive(Debug)]
pub struct Puzzle(Vec<Machine>);

fn movement(input: &str) -> IResult<&str, V2> {
    map(
        separated_pair(
            preceded(tag("X+"), u32),
            tag(", "),
            preceded(tag("Y+"), u32),
        ),
        |(x, y)| V2 {
            x: x as f64,
            y: y as f64,
        },
    )(input)
}

fn button(input: &str) -> IResult<&str, V2> {
    preceded(
        terminated(preceded(tag("Button "), anychar), tag(": ")),
        movement,
    )(input)
}

fn buttons(input: &str) -> IResult<&str, Buttons> {
    map(separated_pair(button, newline, button), |(a, b)| Buttons {
        a,
        b,
    })(input)
}

fn position(input: &str) -> IResult<&str, V2> {
    map(
        separated_pair(
            preceded(tag("X="), u32),
            tag(", "),
            preceded(tag("Y="), u32),
        ),
        |(x, y)| V2 {
            x: x as f64,
            y: y as f64,
        },
    )(input)
}

fn prize(input: &str) -> IResult<&str, V2> {
    preceded(tag("Prize: "), position)(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    map(
        separated_pair(buttons, newline, prize),
        |(buttons, prize)| Machine { buttons, prize },
    )(input)
}

fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    map(
        separated_list1(newline, terminated(machine, opt(newline))),
        Puzzle,
    )(input)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Puzzle {
    puzzle(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Puzzle) -> f64 {
    input
        .0
        .iter()
        .filter_map(|m| solve_machine(m, 0.))
        .map(|s| s.a * 3. + s.b)
        .sum()
}

#[derive(Debug)]
struct Solution {
    a: f64,
    b: f64,
}

fn solve_machine(machine: &Machine, offset: f64) -> Option<Solution> {
    let mut buttons = array![
        [
            machine.buttons.a.x,
            machine.buttons.b.x,
            machine.prize.x + offset,
        ],
        [
            machine.buttons.a.y,
            machine.buttons.b.y,
            machine.prize.y + offset,
        ],
    ];
    if buttons[[0, 0]] == 0. {
        let mut row = buttons.row_mut(0);
        row += 1.;
    }
    for j in 0..buttons.len_of(Axis(0)) {
        let v = buttons[[j, j]];
        if v != 1. {
            let mut row = buttons.row_mut(j);
            row /= v;
        }
        for i in 0..buttons.len_of(Axis(0)) {
            if i != j {
                let sub = &buttons.row(j) * buttons[[i, j]];
                let mut row = buttons.row_mut(i);
                row -= &sub;
            }
        }
    }
    let a = buttons[[0, 2]].round();
    let b = buttons[[1, 2]].round();
    if a * machine.buttons.a.x + b * machine.buttons.b.x == machine.prize.x + offset
        && a * machine.buttons.a.y + b * machine.buttons.b.y == machine.prize.y + offset
    {
        Some(Solution { a, b })
    } else {
        None
    }
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Puzzle) -> f64 {
    input
        .0
        .iter()
        .filter_map(|m| solve_machine(m, 10000000000000.))
        .map(|s| s.a * 3. + s.b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test_case(TEST => 480.)]
    fn part1(input: &str) -> f64 {
        solve_part1(&input_generator(input))
    }

    #[test_case(TEST, 0 => Some(280.))]
    #[test_case(TEST, 1 => None)]
    #[test_case(TEST, 2 => Some(200.))]
    #[test_case(TEST, 3 => None)]
    fn part1_single(input: &str, idx: usize) -> Option<f64> {
        let puzzle = input_generator(input);
        solve_machine(&puzzle.0[idx], 0.).map(|s| s.a * 3. + s.b)
    }

    // #[test_case(TEST => 80)]
    // fn part2(input: &str) -> usize {
    //     solve_part2(&input_generator(input))
    // }
}
