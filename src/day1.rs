use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BinaryHeap;

type Input = BinaryHeap<u32>;

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|s| s.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut clone = input.clone();
    clone.pop().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let mut calorie_list = input.clone();

    let mut sum = 0;

    for _ in 0..3 {
        if let Some(top) = calorie_list.pop() {
            sum += top;
        }
    }

    sum
}
