use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded},
    IResult,
};

type Input = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    NoOp,
    Addx(i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, result) = alt((tag("noop"), tag("addx")))(input)?;
    Ok(match result {
        "noop" => (input, Instruction::NoOp),
        "addx" => {
            let (input, amount) = preceded(tag(" "), complete::i32)(input)?;

            (input, Instruction::Addx(amount))
        }
        _ => unreachable!(),
    })
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, output) = separated_list1(line_ending, parse_instruction)(input)?;

    Ok((input, output))
}

#[aoc_generator(day10)]
pub fn day10_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    input
        .iter()
        .flat_map(|ins| match ins {
            Instruction::NoOp => vec![0],
            Instruction::Addx(a) => vec![0, *a],
        })
        .scan(1, |scan, inc| {
            let tmp = *scan;
            *scan += inc;
            Some(tmp)
        })
        .enumerate()
        .filter(|&(index, _)| (index + 21) % 40 == 0)
        .map(|(i, value)| (1 + i as i32) * value)
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Input) -> String {
    (0..)
        .map(|index| (index % 40) + 1)
        .zip(
            input
                .iter()
                .flat_map(|ins| match ins {
                    Instruction::NoOp => vec![0],
                    Instruction::Addx(a) => vec![0, *a],
                })
                .scan(1, |scan, inc| {
                    let tmp = *scan;
                    *scan += inc;
                    Some(tmp)
                }),
        )
        .map(|(index, pos)| {
            if index == pos || index == pos + 1 || index == pos + 2 {
                '#'
            } else {
                ' '
            }
        })
        .enumerate()
        .flat_map(|(i, c)| if i % 40 == 0 { vec!['\n', c] } else { vec![c] })
        .collect::<String>()
}
