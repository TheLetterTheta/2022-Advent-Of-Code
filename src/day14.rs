use aoc_runner_derive::{aoc, aoc_generator};
use colored::Colorize;
use itertools::{Itertools, MinMaxResult::MinMax};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Point(usize, usize);

#[derive(Debug, Copy, Clone)]
pub struct Line(Point, Point);

type Input = Vec<Line>;

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, point) = separated_pair(complete::u32, tag(","), complete::u32)(input)?;

    Ok((input, Point(point.0 as usize, point.1 as usize)))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, points) = separated_list1(tag(" -> "), parse_point)(input)?;

    let lines = points
        .iter()
        .tuple_windows()
        .map(|(l, r)| {
            if l.0 == r.0 {
                Line(Point(l.0, l.1.min(r.1)), Point(l.0, l.1.max(r.1)))
            } else {
                Line(Point(l.0.min(r.0), l.1), Point(l.0.max(r.0), l.1))
            }
        })
        .collect::<Vec<Line>>();
    Ok((input, lines))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_lines)(input)?;
    let list = list.into_iter().flatten().collect();

    Ok((input, list))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Simulation {
    Sand,
    Wall,
}

#[aoc_generator(day14)]
pub fn day14_generator(input: &str) -> Input {
    let (remainder, input) = parse_input(input).unwrap();
    assert!(remainder.is_empty());
    input
}

struct Grid<'a>(&'a Vec<Vec<Option<Simulation>>>);

impl<'a> Display for Grid<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "┏{0}┓\n{1}\n┗{0}┛",
            ("━".bright_white().bold()).repeat(self.0.first().map(|f| f.len()).unwrap_or(0)),
            self.0
                .iter()
                .map(|l| format!(
                    "{1}{}{1}",
                    l.iter()
                        .map(|p| match p {
                            Some(Simulation::Wall) => "▉".white(),
                            Some(Simulation::Sand) => "ஃ".bright_yellow().bold(),
                            None => " ".clear(),
                        })
                        .join(""),
                    "┃".bright_white().bold()
                ))
                .join("\n")
        )?;
        Ok(())
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let (min_x, max_x) = match input
        .iter()
        .flat_map(|line| [line.0 .0, line.1 .0])
        .chain(std::iter::once(500))
        .minmax()
    {
        MinMax(min, max) => (min, max),
        _ => unreachable!(),
    };

    let max_y = input
        .iter()
        .flat_map(|line| [line.0 .1, line.1 .1])
        .max()
        .unwrap();

    let mut grid: Vec<Vec<Option<Simulation>>> = vec![vec![None; 1 + (max_x - min_x)]; 1 + (max_y)];

    for line in input.iter() {
        // All lines are straight
        if line.0 .0 == line.1 .0 {
            // Same x's
            for item in grid.iter_mut().take(line.1.1 + 1).skip(line.0.1) {
                item[line.0 .0 - min_x] = Some(Simulation::Wall);
            }
        } else {
            // Guranteed to have same y's
            for x in line.0 .0..=line.1 .0 {
                grid[line.0 .1][x - min_x] = Some(Simulation::Wall);
            }
        }
    }

    'outer: loop {
        let mut pos_x = 500;
        for y in 0..max_y {
            // look down
            if grid[y + 1][pos_x - min_x].is_some() {
                if pos_x - min_x > 0 {
                    match grid[y + 1][pos_x - (1 + min_x)] {
                        Some(_) => {
                            if pos_x < max_x - 1 {
                                match grid[y + 1][1 + pos_x - min_x] {
                                    Some(_) => {
                                        // nowhere below to place sand
                                        grid[y][pos_x - min_x] = Some(Simulation::Sand);
                                        continue 'outer;
                                    }
                                    None => {
                                        pos_x += 1;
                                        continue;
                                    }
                                }
                            } else {
                                // Exit condition - sand, and fall off side
                                break 'outer;
                            }
                        }
                        // Sand can go left
                        None => {
                            pos_x -= 1;
                            continue;
                        }
                    }
                } else {
                    // Exit condition - sand, and fall off side
                    break 'outer;
                }
            }

            if y == max_y - 1 {
                // We've checked, and can place no more sand
                break 'outer;
            }
        }
    }

    println!("{}", Grid(&grid));

    grid.iter()
        .map(|line| {
            line.iter()
                .filter(|p| matches!(p, Some(Simulation::Sand)))
                .count()
        })
        .sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let max_y = input
        .iter()
        .flat_map(|line| [line.0 .1, line.1 .1])
        .max()
        .unwrap();

    let floor_y = max_y + 2;
    let floor_len = 2 * (2 + max_y);

    let mut input = input.clone();
    input.push(Line(
        Point(500 - (floor_len / 2), floor_y),
        Point(500 + (floor_len / 2), floor_y),
    ));
    let input = input;

    let max_y = input
        .iter()
        .flat_map(|line| [line.0 .1, line.1 .1])
        .chain(std::iter::once(0))
        .max()
        .unwrap();

    let (min_x, max_x) = match input
        .iter()
        .flat_map(|line| [line.0 .0, line.1 .0])
        .minmax()
    {
        MinMax(min, max) => (min, max),
        _ => unreachable!(),
    };

    let mut grid: Vec<Vec<Option<Simulation>>> = vec![vec![None; 1 + (max_x - min_x)]; 1 + (max_y)];

    for line in input.iter() {
        // All lines are straight
        if line.0 .0 == line.1 .0 {
            // Same x's
            for item in grid.iter_mut().take(line.1 .1 + 1).skip(line.0 .1) {
                item[line.0 .0 - min_x] = Some(Simulation::Wall);
            }
        } else {
            // Guranteed to have same y's
            for x in line.0 .0..=line.1 .0 {
                grid[line.0 .1][x - min_x] = Some(Simulation::Wall);
            }
        }
    }

    let mut stack = vec![(0, 500 - min_x)];
    let mut count = 0;
    while let Some(point) = stack.pop() {
        count += 1;

        grid[point.0][point.1] = Some(Simulation::Sand);
        
        if point.0 < max_y - 1 {

            if point.1 < max_x - 1 && grid[point.0+1][point.1 + 1].is_none() {
                stack.push((point.0 + 1, point.1 + 1));
            }
            if point.0 > 0 && grid[point.0 + 1][point.1 - 1].is_none() {
                stack.push((point.0 + 1, point.1 - 1));
            }
            if grid[point.0 + 1][point.1].is_none() {
                stack.push((point.0 + 1, point.1));
            }
        }
    }
    count
}
