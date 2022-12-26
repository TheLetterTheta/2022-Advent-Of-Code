use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

type Input = Vec<i64>;

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(line_ending, complete::i64)(input)
}

#[aoc_generator(day20)]
pub fn day20_generator(input: &str) -> Input {
    let _input = "1
2
-3
3
-2
0
4";
    let (input, output) = parse_input(input).unwrap();
    assert!(input.is_empty());
    output
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let input = input.iter().enumerate().collect_vec();
    let mut output = input.clone();
    input.iter().for_each(|(id, _)| {
        let index = output
            .iter()
            .position(|output_value| output_value.0 == *id)
            .unwrap();

        let current = output.remove(index);
        let added = index as i64 + current.1;
        let new_index = added.rem_euclid(output.len() as i64);

        output.insert(new_index as usize, current);
    });

    let zero_index = output.iter().position(|v| *v.1 == 0).unwrap();
    let a = output[(1000 + zero_index) % output.len()].1;
    let b = output[(2000 + zero_index) % output.len()].1;
    let c = output[(3000 + zero_index) % output.len()].1;

    a + b + c
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    let input = input
        .iter()
        .map(|v| v * 811_589_153)
        .enumerate()
        .collect_vec();
    let mut output = input.clone();

    for _ in 0..10 {
        input.iter().for_each(|(id, _)|  {
            let index = output
                .iter()
                .position(|output_value| output_value.0 == *id)
                .unwrap();

            let current = output.remove(index);
            let added = index as i64 + current.1;
            let new_index = added.rem_euclid(output.len() as i64);

            output.insert(new_index as usize, current);
        });
    }

    let zero_index = output.iter().position(|v| v.1 == 0).unwrap();
    let a = output[(1000 + zero_index) % output.len()].1;
    let b = output[(2000 + zero_index) % output.len()].1;
    let c = output[(3000 + zero_index) % output.len()].1;

    a + b + c
}
