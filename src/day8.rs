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
    for (i, row) in input.iter().enumerate().take(input.len() - 1).skip(1) {
        let mut max = row[0];

        for (j, point) in row.iter().enumerate().take(row.len() - 1).skip(1) {
            if point > &max {
                max = *point;
                counted.insert((i, j));
            }
        }
    }

    // right
    for (i, row) in input.iter().enumerate().take(input.len() - 1).skip(1) {
        let mut max = row[input[i].len() - 1];

        for j in (1..row.len() - 1).rev() {
            let point = row[j];
            if point > max {
                max = point;
                counted.insert((i, j));
            }
        }
    }

    // top
    for j in 1..(input.len() - 1) {
        let mut max = input[0][j];

        for (i, point) in input
            .iter()
            .enumerate()
            .take(input[j].len() - 1)
            .skip(1)
            .map(|(i, r)| (i, r[j]))
            .take_while(move |_| max < 9)
        {
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

    for (i, row) in input.iter().enumerate().take(input.len() - 1).skip(1) {
        for (j, curr) in row.iter().enumerate().take(row.len() - 1).skip(1) {
            let mut curr_score = 0;

            // compute score

            curr_score += (0..i)
                .rev()
                .map(|k| input[k][j])
                .take_while(|&check| check < *curr)
                .count();
            curr_score *= input
                .iter()
                .skip(i + 1)
                .map(|row| row[j])
                .take_while(|&check| check < *curr)
                .count();
            curr_score *= (0..j)
                .rev()
                .map(|k| input[i][k])
                .take_while(|&check| check < *curr)
                .count();

            let mut dist = 0;

            for k in (j + 1)..input[i].len() {
                let check = input[i][k];

                dist += 1;
                if check >= *curr {
                    break;
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
