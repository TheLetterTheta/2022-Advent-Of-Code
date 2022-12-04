use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<((u32, u32), (u32, u32))>;

#[aoc_generator(day4)]
pub fn day4_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            let (one, two) = left.split_once("-").unwrap();
            let (three, four) = right.split_once("-").unwrap();

            (
                (one.parse().unwrap(), two.parse().unwrap()),
                (three.parse().unwrap(), four.parse().unwrap()),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(left, right)| {
            (right.0 <= left.0 && right.1 >= left.1) || (left.0 <= right.0 && left.1 >= right.1)
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(left, right)| (left.1 < right.0 && right.1 < left.0))
        .count()
}
