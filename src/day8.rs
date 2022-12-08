use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Input = Vec<Vec<u32>>;

#[aoc_generator(day8)]
pub fn day8_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut counted = HashSet::new();

    // left
    for i in 1..(input.len() - 1) {
        let mut max = input[i][0];

        for j in 1..(input[i].len() - 1) {
            let point = input[i][j];
            if point > max {
                max = point;
                counted.insert((i, j));
            }
        }
    }

    // right
    for i in 1..(input.len() - 1) {
        let mut max = input[i][input[i].len() - 1];

        for j in (1..input[i].len() - 1).rev() {
            let point = input[i][j];
            if point > max {
                max = point;
                counted.insert((i, j));
            }
        }
    }

    // top
    for j in 1..(input.len() - 1) {
        let mut max = input[0][j];

        for i in 1..input[j].len() - 1 {
            let point = input[i][j];
            if point > max {
                max = point;
                counted.insert((i, j));
            }
        }
    }

    // bottom
    for j in 1..(input.len() - 1) {
        let mut max = input[input.len() - 1][j];

        for i in (1..(input.len() - 1)).rev() {
            let point = input[i][j];
            if point > max {
                max = point;
                counted.insert((i, j));
            }
        }
    }

    let mut count = counted.len();

    count += 2 * input.len();
    count += 2 * input[0].len();
    count -= 4;

    count
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut score = 0;

    for i in 1..(input.len() - 1) {
        for j in 1..(input.len() - 1) {
            let curr = input[i][j];
            let mut curr_score = 0;

            // compute score
            let mut dist = 0;
            for k in (0..i).rev() {
                let check = input[k][j];

                if check >= curr {
                    dist += 1;
                    break;
                } else {
                    dist += 1;
                }
            }

            curr_score += dist;
            dist = 0;

            for check in input.iter().skip(i + 1).map(|row| row[j]) {
                if check >= curr {
                    dist += 1;
                    break;
                } else {
                    dist += 1;
                }
            }

            curr_score *= dist;
            dist = 0;

            for k in (j + 1)..input[i].len() {
                let check = input[i][k];

                if check >= curr {
                    dist += 1;
                    break;
                } else {
                    dist += 1;
                }
            }

            curr_score *= dist;
            dist = 0;

            for k in (0..j).rev() {
                let check = input[i][k];

                if check >= curr {
                    dist += 1;
                    break;
                } else {
                    dist += 1;
                }
            }

            curr_score *= dist;

            if score < curr_score {
                score = curr_score;
            }
        }
    }

    score
}
