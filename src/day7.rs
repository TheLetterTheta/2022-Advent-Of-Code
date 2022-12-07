use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, not_line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug, PartialEq)]
pub enum Line {
    Command(Command),
    File(String, usize),
    Directory(String),
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("ls")(input)?;

    Ok((input, Command::Ls))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, (_, path)) = tuple((tag("cd "), not_line_ending))(input)?;

    Ok((input, Command::Cd(path.to_string())))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_ls, parse_cd))(input)
}

fn parse_line_command(input: &str) -> IResult<&str, Line> {
    let (input, (_, command)) = tuple((tag("$ "), parse_command))(input)?;

    Ok((input, Line::Command(command)))
}

fn parse_file(input: &str) -> IResult<&str, Line> {
    let (input, (size, _, filename)) = tuple((
        map_res(digit1, |s: &str| s.parse::<usize>()),
        tag(" "),
        not_line_ending,
    ))(input)?;

    Ok((input, Line::File(filename.to_string(), size)))
}

fn parse_directory(input: &str) -> IResult<&str, Line> {
    let (input, (_, directory)) = tuple((tag("dir "), alpha1))(input)?;

    Ok((input, Line::Directory(directory.to_string())))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((parse_line_command, parse_file, parse_directory))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, parse_line)(input)
}

type Input = Vec<Line>;

#[aoc_generator(day7)]
pub fn day7_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut master_set: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut current_path = vec![];

    input.iter().for_each(|line| {
        match line {
            Line::Command(Command::Cd(path)) => {
                let path = path.as_str();
                // Navigate to new directory
                match path {
                    ".." => {
                        current_path.pop();
                    }
                    "/" => {
                        current_path.clear();
                    }
                    _ => {
                        current_path.push(path);
                    }
                }
            }
            Line::File(_, size) => {
                // Add file to current path
                master_set
                    .entry(current_path.clone())
                    .and_modify(|total_size| *total_size += size)
                    .or_insert(*size);
            }
            Line::Command(Command::Ls) | Line::Directory(_) => {} // New directory in this folder
        }
    });
    current_path.clear();

    let mut total_sizes: HashMap<Vec<&str>, usize> = HashMap::new();

    for (path, size) in master_set.iter() {
        let mut paths = path.clone();

        loop {
            total_sizes
                .entry(paths.clone())
                .and_modify(|total_size| *total_size += size)
                .or_insert(*size);

            if paths.pop().is_none() {
                break;
            }
        }
    }

    total_sizes.values().filter(|&&v| v < 100_000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut master_set: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut current_path = vec![];

    input.iter().for_each(|line| {
        match line {
            Line::Command(Command::Cd(path)) => {
                let path = path.as_str();
                // Navigate to new directory
                match path {
                    ".." => {
                        current_path.pop();
                    }
                    "/" => {
                        current_path.clear();
                    }
                    _ => {
                        current_path.push(path);
                    }
                }
            }
            Line::File(_, size) => {
                // Add file to current path
                master_set
                    .entry(current_path.clone())
                    .and_modify(|total_size| *total_size += size)
                    .or_insert(*size);
            }
            Line::Command(Command::Ls) | Line::Directory(_) => {} // New directory in this folder
        }
    });
    current_path.clear();

    let mut total_sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut used_space = 0;

    for (path, size) in master_set.iter() {
        used_space += size;
        let mut paths = path.clone();

        loop {
            total_sizes
                .entry(paths.clone())
                .and_modify(|total_size| *total_size += size)
                .or_insert(*size);

            if paths.pop().is_none() {
                break;
            }
        }
    }

    let unused_space = 70_000_000 - used_space;
    let needed_space = 30_000_000 - unused_space;

    *total_sizes
        .values()
        .filter(|&&s| s >= needed_space)
        .min_by_key(|s| *s - needed_space)
        .unwrap()
}
