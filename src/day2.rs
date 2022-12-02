use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(u32, u32)>;

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (them, you) = l.split_once(" ").unwrap();
            let them = match them {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => unreachable!(),
            };
            let you = match you {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };

            (them, you)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input
        .into_iter()
        .map(|&(them, you)| {
            if them == (you - 1) || (them == 3 && you == 1) {
                you + 6
            } else if them == you {
                you + 3
            } else {
                you
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|(them, you)| {
            match you {
                1 => {
                    // Lose
                    if them - 1 == 0 {
                        3
                    } else {
                        them - 1
                    }
                }
                2 => {
                    // Draw
                    them + 3
                }
                3 => {
                    // Win
                    if them + 1 == 4 {
                        7
                    } else {
                        them + 7
                    }
                }
                _ => unreachable!(),
            }
        })
        .sum()
}
