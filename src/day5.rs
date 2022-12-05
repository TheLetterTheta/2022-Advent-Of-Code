use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

type Input = (Vec<Vec<char>>, Vec<(u32, usize, usize)>);

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

    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    (
        stacks,
        input
            .lines()
            .skip(10)
            .map(|line| {
                let captures = RE.captures(line).unwrap();
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

    for (num, from, to) in instructions.iter() {
        for _ in 0..*num {
            let popped = state[*from].pop().unwrap();
            state[*to].push(popped);
        }
    }
    state.iter().map(|m| m.iter().rev().next().unwrap()).collect::<String>()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let (mut state, instructions) = input.clone();

    for (num, from, to) in instructions.iter() {
        let mut intermediate = vec![];
        for _ in 0..*num {
            let popped = state[*from].pop().unwrap();
            intermediate.push(popped);
        }

        while let Some(i) = intermediate.pop() {
            state[*to].push(i);
        }
    }
    state.iter().map(|m| m.iter().rev().next().unwrap()).collect::<String>()
}
