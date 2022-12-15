use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sensor {
    pub position: (i64, i64),
    pub nearest_beacon: (i64, i64),
}

fn point_distance(from: (i64, i64), to: (i64, i64)) -> u64 {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub slope: i64,
    pub b: i64,
    pub domain: (i64, i64),
    pub range: (i64, i64),
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Vec<(i64, i64)>> {
        let min_domain = (
            self.domain.0.max(other.domain.0),
            self.domain.1.min(other.domain.1),
        );
        if min_domain.1 < min_domain.0 {
            // Domains don't overlap
            return None;
        }

        let min_range = (
            self.range.0.max(other.range.0),
            self.range.1.min(other.range.1),
        );
        if min_range.1 < min_range.0 {
            // Ranges don't overlap
            return None;
        }

        let (a, c, b, d) = (self.slope, self.b, other.slope, other.b);
        if a == b {
            // Parallel lines
            if c == d {
                None
            } else {
                Some(
                    (self.domain.0.max(other.domain.0)..self.domain.1.min(other.domain.1))
                        .zip(self.range.0.max(other.range.0)..self.range.1.min(other.range.1))
                        .collect_vec(),
                )
            }
        } else {
            Some(vec![((d - c) / (a - b), a * ((d - c) / a - b) + c)])
        }
    }
}

impl Sensor {
    pub fn beacon_distance(&self) -> u64 {
        point_distance(self.position, self.nearest_beacon)
    }

    /// Given a sensor at a point (x,y) with a Manhattan Distance of D
    /// And a line (y), the segment of intersection is:
    /// let offset = D - Distance(sensor y, line y)
    /// The line segment therefore is [x - offset, x + offset]
    pub fn overlap_y(&self, point: i64) -> Option<(i64, i64)> {
        let offset = (self.beacon_distance() as i64) - (self.position.1.abs_diff(point) as i64);
        if offset > 0 {
            Some((self.position.0 - offset, self.position.0 + offset))
        } else {
            None
        }
    }

    pub fn point_in_range(&self, point: (i64, i64)) -> bool {
        point_distance(self.position, point) <= self.beacon_distance()
    }

    pub fn intersection(&self, other: &Self) -> impl Iterator<Item = (i64, i64)> {
        let self_checks = self.outer_perimiter();
        let other_checks = other.outer_perimiter();

        self_checks.into_iter().flat_map(move |check| {
            other_checks
                .into_iter()
                .filter_map(move |o| o.intersection(&check))
                .flatten()
        })
    }

    pub fn outer_perimiter(&self) -> [Line; 4] {
        self.perimiter(self.beacon_distance() as i64 + 1)
    }

    pub fn perimiter(&self, dist: i64) -> [Line; 4] {
        // top
        let top_left = Line {
            slope: 1,
            b: self.position.1 - (self.position.0 - dist),
            domain: (self.position.0 - dist, self.position.0),
            range: (self.position.1, self.position.1 + dist),
        };
        let top_right = Line {
            slope: -1,
            b: self.position.1 + dist + self.position.0,
            domain: (self.position.0, self.position.0 + dist),
            range: (self.position.1, self.position.1 + dist),
        };
        let bottom_right = Line {
            slope: 1,
            b: self.position.1 - (dist + self.position.0),
            domain: (self.position.0, self.position.0 + dist),
            range: (self.position.1 - dist, self.position.1),
        };
        let bottom_left = Line {
            slope: -1,
            b: self.position.1 + (self.position.0 - dist),
            domain: (self.position.0 - dist, self.position.0),
            range: (self.position.1 - dist, self.position.1),
        };

        [top_left, top_right, bottom_right, bottom_left]
    }
}

type Input = Vec<Sensor>;

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
pub fn solve_part2(input: &Input) -> i64 {
    let Some((x, y)) = input
        .iter()
        .combinations(2)
        .flat_map(|compare| compare[0].intersection(compare[1]))
        .filter(|&p| p.0 >= 0 && p.0 <= 4_000_000 && p.1 >= 0 && p.1 <= 4_000_000)
        .find(|point| !input.iter().any(|sensor| sensor.point_in_range(*point))) else {
            panic!("No Solution!");
        };

    x * 4_000_000 + y
}
