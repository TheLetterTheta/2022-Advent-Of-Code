use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Input = (Vec<Vec<char>>, Vec<(u32, usize, usize)>);

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[aoc_generator(day5)]
pub fn day5_generator(input: &str) -> Input {
    let stacks = vec![
        "GJZ".chars().rev().collect(),
        "CVFWPRLQ".chars().rev().collect(),
        "RGLCMPF".chars().rev().collect(),
        "MHPWBFL".chars().rev().collect(),
        "QVSFCG".chars().rev().collect(),
        "LTQMZJHW".chars().rev().collect(),
        "VBSFH".chars().rev().collect(),
        "SZJF".chars().rev().collect(),
        "TBHFPDCM".chars().rev().collect(),
    ];

    let expr: &Regex = regex!(r"move (\d+) from (\d+) to (\d+)");

    (
        stacks,
        input
            .lines()
            .skip(10)
            .map(|line| {
                let captures = expr.captures(line).unwrap();
                (
                    captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                    captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
                )
            })
            .collect(),
    )
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> String {
    let (mut state, instructions) = input.clone();

    for (num, from, to) in instructions {
        for _ in 0..num {
            let popped = state[from].pop().unwrap();
            state[to].push(popped);
        }
    }
    state.iter().map(|m| m.last().unwrap()).collect::<String>()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let (mut state, instructions) = input.clone();

    for (num, from, to) in instructions {
        let mut intermediate = vec![];
        for _ in 0..num {
            let popped = state[from].pop().unwrap();
            intermediate.push(popped);
        }

        while let Some(i) = intermediate.pop() {
            state[to].push(i);
        }
    }
    state.iter().map(|m| m.last().unwrap()).collect::<String>()
}
