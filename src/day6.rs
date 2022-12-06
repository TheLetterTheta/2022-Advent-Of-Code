use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<char>;

#[aoc_generator(day6)]
pub fn day6_generator(input: &str) -> Input {
    input.chars().collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .windows(4)
        .enumerate()
        .find(|(_, v)| v.iter().unique().count() == 4)
        .unwrap()
        .0
        + 4
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> usize {
    input
        .windows(14)
        .enumerate()
        .find(|(_, v)| v.iter().unique().count() == 14)
        .unwrap()
        .0
        + 14
}
