use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
pub struct Sensor {
    pub position: (i64, i64),
    pub nearest_beacon: (i64, i64),
}

fn point_distance(from: (i64, i64), to: (i64, i64)) -> u64 {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

impl Sensor {
    pub fn beacon_distance(&self) -> u64 {
        point_distance(self.position, self.nearest_beacon)
    }

    pub fn overlap_y(&self, point: i64) -> Option<(i64, i64)> {
        let offset = (self.beacon_distance() as i64) - (self.position.1.abs_diff(point) as i64);
        if offset > 0 {
            Some((self.position.0 - offset, self.position.0 + offset))
        } else {
            None
        }
    }
}

type Input = Vec<Sensor>;

/// Given a sensor at a point (x,y) with a Manhattan Distance of D
/// And a line (y), the segment of intersection is:
/// let offset = D - Distance(sensor y, line y)
/// The line segment therefore is [x - offset, x + offset]

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, ((position_x, position_y), (beacon_x, beacon_y))) = preceded(
        tag("Sensor at x="),
        separated_pair(
            separated_pair(complete::i64, tag(", y="), complete::i64),
            tag(": closest beacon is at x="),
            separated_pair(complete::i64, tag(", y="), complete::i64),
        ),
    )(input)?;

    Ok((
        input,
        Sensor {
            position: (position_x, position_y),
            nearest_beacon: (beacon_x, beacon_y),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_sensor)(input)?;

    Ok((input, list))
}

#[aoc_generator(day15)]
pub fn day15_generator(input: &str) -> Input {
    let (input, output) = parse_input(input).unwrap();
    assert!(input.is_empty());
    output
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    let mut overlap_gaps: Vec<(i64, i64)> = vec![];

    let mut gap_iter = input
        .iter()
        .filter_map(|b| b.overlap_y(2_000_000))
        .sorted_unstable();

    let (mut start, mut end) = gap_iter.next().expect("No elements");

    for (next_start, next_end) in gap_iter {
        if next_start <= end {
            end = end.max(next_end);
        } else {
            overlap_gaps.push((start, end));
            start = next_start;
            end = next_end;
        }
    }
    overlap_gaps.push((start, end));

    overlap_gaps.iter().map(|(l, r)| l.abs_diff(*r)).sum()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Input) -> usize {
    todo!();
}
