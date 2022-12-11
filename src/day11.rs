use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use num::integer::lcm;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    remainder: u64,
    false_throw: usize,
    true_throw: usize,
    inspected: usize,
}

type Input = Vec<Monkey>;

fn parse_square(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* old")(input)?;
    Ok((input, Operation::Square))
}

fn parse_add(input: &str) -> IResult<&str, Operation> {
    let (input, n) = preceded(tag("+ "), complete::u64)(input)?;
    Ok((input, Operation::Add(n)))
}

fn parse_multiply(input: &str) -> IResult<&str, Operation> {
    let (input, n) = preceded(tag("* "), complete::u64)(input)?;
    Ok((input, Operation::Mult(n)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = preceded(tag("Monkey "), complete::u8)(input)?;
    let (input, items) = preceded(
        tuple((tag(":"), line_ending, space0, tag("Starting items: "))),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, operation) = preceded(
        tuple((line_ending, space0, tag("Operation: new = old "))),
        alt((parse_add, parse_multiply, parse_square)),
    )(input)?;
    let (input, remainder) = preceded(tag("\n  Test: divisible by "), complete::u64)(input)?;

    let (input, true_throw) = preceded(
        tuple((line_ending, space0, tag("If true: throw to monkey "))),
        complete::u8,
    )(input)?;
    let (input, false_throw) = preceded(
        tuple((line_ending, space0, tag("If false: throw to monkey "))),
        complete::u8,
    )(input)?;

    let true_throw = true_throw as usize;
    let false_throw = false_throw as usize;

    Ok((
        input,
        Monkey {
            items,
            operation,
            remainder,
            true_throw,
            false_throw,
            inspected: 0,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(tag("\n\n"), parse_monkey)(input)
}

#[aoc_generator(day11)]
pub fn day11_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut input = input.clone();
    for _ in 0..20 {
        for i in 0..input.len() {
            let new_items = input[i]
                .items
                .iter()
                .map(|&x| match input[i].operation {
                    Operation::Add(n) => n + x,
                    Operation::Mult(n) => n * x,
                    Operation::Square => x * x,
                })
                .map(|n| n / 3);
            let (mut throw_true, mut throw_false): (Vec<u64>, Vec<u64>) =
                new_items.partition(|&x| x % input[i].remainder == 0);

            let t = input[i].true_throw;
            let f = input[i].false_throw;

            input[i].inspected += input[i].items.len();
            input[i].items.clear();
            input[t].items.append(&mut throw_true);
            input[f].items.append(&mut throw_false);
        }
    }

    input.sort_unstable_by_key(|m| m.inspected);
    input.iter().rev().take(2).map(|m| m.inspected).product()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let lcm = input.iter().map(|m| m.remainder).fold(1, lcm);

    let mut input = input.clone();
    for _ in 0..10000 {
        for i in 0..input.len() {
            let new_items = input[i]
                .items
                .iter()
                .map(|&x| match input[i].operation {
                    Operation::Add(n) => n + x,
                    Operation::Mult(n) => n * x,
                    Operation::Square => x * x,
                })
                .map(|n| n % lcm);
            let (mut throw_true, mut throw_false): (Vec<u64>, Vec<u64>) =
                new_items.partition(|&x| x % input[i].remainder == 0);

            let t = input[i].true_throw;
            let f = input[i].false_throw;

            input[i].inspected += input[i].items.len();
            input[i].items.clear();
            input[t].items.append(&mut throw_true);
            input[f].items.append(&mut throw_false);
        }
    }

    input.sort_unstable_by_key(|m| m.inspected);
    input.iter().rev().take(2).map(|m| m.inspected).product()
}
