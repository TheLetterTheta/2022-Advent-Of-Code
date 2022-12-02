use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u64>>;

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<u64>().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|cal| cal.iter().sum::<u64>())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let mut calorie_list = input
        .iter()
        .map(|cal| cal.iter().sum::<u64>())
        .collect::<Vec<_>>();
    calorie_list.sort_unstable();

    calorie_list.iter().rev().take(3).sum()
}
