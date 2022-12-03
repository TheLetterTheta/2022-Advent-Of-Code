use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

type Input = Vec<(String, String)>;

#[aoc_generator(day3)]
pub fn day3_generator(input: &str) -> Input {
    input
        .lines()
        .map(|mut l| {
            let half = l.len() / 2;
            let (left, right) = l.split_at(half);
            (left.to_string(), right.to_string())
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|(left, right)| {
            let mut dupe = left.chars().collect::<HashSet<char>>();

            for c in right.chars() {
                if dupe.contains(&c) {
                    return c;
                }
            }
            unreachable!();
        })
        .map(|c| {
            if c.is_lowercase() {
                (c as u8 - 96) as u32
            } else {
                (c as u8 - (64 - 26)) as u32
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let mut sum = 0;
    for chunk in &input
        .iter()
        .map(|(left, right)| {
            let full = format!("{}{}", left, right);
            full.chars().collect::<HashSet<char>>()
        })
        .chunks(3)
    {
        let mut chunk = chunk.collect::<Vec<_>>();
        let first = &chunk[0];
        let second = &chunk[1];
        let third = &chunk[2];

        for s in first.intersection(&second) {
            if third.contains(&s) {
                if s.is_lowercase() {
                    sum += (*s as u8 - 96) as u32;
                } else {
                    sum += (*s as u8 - (64 - 26)) as u32;
                }
            }
        }
    }
    sum
}
