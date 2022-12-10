use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub enum Direction {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, (direction, size)) = separated_pair(
        alt((
            complete::char('L'),
            complete::char('R'),
            complete::char('U'),
            complete::char('D'),
        )),
        tag(" "),
        complete::u8,
    )(input)?;
    Ok((
        input,
        match direction {
            'L' => Direction::Left(size),
            'R' => Direction::Right(size),
            'U' => Direction::Up(size),
            'D' => Direction::Down(size),
            _ => unreachable!(),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, parse_direction)(input)
}

fn update_direction(to: (i16, i16), from: (i16, i16)) -> (i16, i16) {
    let mut new = from;

    if to.1 == from.1 || to.0 == from.0 {
        if to.1 > from.1 + 1 {
            new.1 += 1;
        } else if to.1 < from.1 - 1 {
            new.1 -= 1;
        }

        if to.0 > from.0 + 1 {
            new.0 += 1;
        } else if to.0 < from.0 - 1 {
            new.0 -= 1;
        }
    } else if to.1.abs_diff(from.0) > 1 && to.0.abs_diff(from.0) > 1 {
        // diagonal
        if to.1 > from.1 {
            new.1 += 1;
        } else {
            new.1 -= 1;
        }

        if to.0 > from.0 {
            new.0 += 1;
        } else {
            new.0 -= 1;
        }
    } else if to.0.abs_diff(from.0) > 1 {
        // x has changed, but y is different
        if to.0 > from.0 + 1 {
            new.0 = to.0 - 1;
        } else {
            new.0 = to.0 + 1;
        }
        new.1 = to.1;
    } else if to.1.abs_diff(from.1) > 1 {
        // y has changed, but y is different
        if to.1 > from.1 + 1 {
            new.1 = to.1 - 1;
        } else {
            new.1 = to.1 + 1;
        }
        new.0 = to.0;
    }

    new
}

type Input = Vec<Direction>;

#[aoc_generator(day9)]
pub fn day9_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut visited = HashSet::new();

    let mut head: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);

    visited.insert(tail);

    for direction in input {
        match direction {
            Direction::Up(n) => {
                for _ in 0..*n {
                    head.0 += 1;
                    tail = update_direction(head, tail);
                    visited.insert(tail);
                }
            }
            Direction::Down(n) => {
                for _ in 0..*n {
                    head.0 -= 1;
                    tail = update_direction(head, tail);
                    visited.insert(tail);
                }
            }
            Direction::Left(n) => {
                for _ in 0..*n {
                    head.1 -= 1;
                    tail = update_direction(head, tail);
                    visited.insert(tail);
                }
            }
            Direction::Right(n) => {
                for _ in 0..*n {
                    head.1 += 1;
                    tail = update_direction(head, tail);
                    visited.insert(tail);
                }
            }
        }
    }
    visited.len()
}

fn update_rope(rope: Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    rope.iter()
        .scan((rope[0], rope[0]), |state, old| {
            state.1 = state.0;
            state.0 = update_direction(state.0, *old);
            Some(*state)
        })
        .map(|piece| piece.0)
        .collect()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut rope = vec![(0, 0); 10];

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for direction in input {
        match direction {
            Direction::Up(n) => {
                for _ in 0..*n {
                    rope[0].1 += 1;

                    rope = update_rope(rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            Direction::Down(n) => {
                for _ in 0..*n {
                    rope[0].1 -= 1;

                    rope = update_rope(rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            Direction::Left(n) => {
                for _ in 0..*n {
                    rope[0].0 -= 1;

                    rope = update_rope(rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            Direction::Right(n) => {
                for _ in 0..*n {
                    rope[0].0 += 1;

                    rope = update_rope(rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
        }
    }
    visited.len()
}
