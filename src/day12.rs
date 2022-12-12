use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Position {
    Start,
    End,
    Middle,
}

type Input = Vec<Vec<(Position, u8)>>;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, output) = separated_list1(
        line_ending,
        many1(nom::combinator::map(none_of("\n"), |c| {
            if c == 'S' {
                (Position::Start, 0)
            } else if c == 'E' {
                (Position::End, 26)
            } else {
                (Position::Middle, c as u8 - b'a')
            }
        })),
    )(input)?;

    Ok((input, output))
}

#[aoc_generator(day12)]
pub fn day12_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let start = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &v)| {
                if v.0 == Position::Start {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .expect("No starting position");
    let end = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &v)| {
                if v.0 == Position::End {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .expect("No starting position");

    //                        Point -> FromPoint
    let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut queue = VecDeque::with_capacity(32);

    queue.push_back(start);

    let input: Vec<Vec<u8>> = input
        .iter()
        .map(|i| i.iter().map(|(_, v)| *v).collect())
        .collect();

    while let Some(curr) = queue.pop_front() {
        let curr_val = input[curr.0][curr.1];

        if curr.1 < input[0].len() - 1
            && input[curr.0][curr.1 + 1] <= curr_val + 1
            && !visited.contains_key(&(curr.0, curr.1 + 1))
        {
            visited.insert((curr.0, curr.1 + 1), curr);
            queue.push_back((curr.0, curr.1 + 1));
        }

        if curr.0 < input.len() - 1
            && input[curr.0 + 1][curr.1] <= curr_val + 1
            && !visited.contains_key(&(curr.0 + 1, curr.1))
        {
            visited.insert((curr.0 + 1, curr.1), curr);
            queue.push_back((curr.0 + 1, curr.1));
        }

        if curr.1 > 0
            && input[curr.0][curr.1 - 1] <= curr_val + 1
            && !visited.contains_key(&(curr.0, curr.1 - 1))
        {
            visited.insert((curr.0, curr.1 - 1), curr);
            queue.push_back((curr.0, curr.1 - 1));
        }

        if curr.0 > 0
            && input[curr.0 - 1][curr.1] <= curr_val + 1
            && !visited.contains_key(&(curr.0 - 1, curr.1))
        {
            visited.insert((curr.0 - 1, curr.1), curr);
            queue.push_back((curr.0 - 1, curr.1));
        }
    }

    let mut answer_key = vec![vec![' '; input[0].len()]; input.len()];

    let mut path = end;
    let mut total = 0;

    while let Some(from) = visited.get(&path) {
        total += 1;

        let print = if from.0 > path.0 {
            '^'
        } else if from.0 < path.0 {
            'v'
        } else if from.1 > path.1 {
            '<'
        } else {
            '>'
        };
        answer_key[from.0][from.1] = print;

        if *from == start {
            break;
        }
        path = *from;
    }
    answer_key[end.0][end.1] = 'E';
    answer_key[start.0][start.1] = 'S';

    println!(
        "{0}\n{1}\n{0}",
        (0..input[0].len() + 2).map(|_| '─').collect::<String>(),
        answer_key
            .iter()
            .map(|line| format!("│{}│", line.iter().collect::<String>()))
            .collect::<Vec<String>>()
            .join("\n")
    );
    total
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut start_options = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &v)| if v.1 == 0 { Some((i, j)) } else { None })
        })
        .collect::<Vec<(usize, usize)>>();

    let end = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &v)| {
                if v.0 == Position::End {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .expect("No starting position");

    let mut min = usize::MAX;
    let mut shortest_print: Vec<Vec<char>> = vec![];
    while let Some(start) = start_options.pop() {
        //                        Point -> FromPoint
        let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut queue = VecDeque::with_capacity(32);

        queue.push_back(start);

        let input: Vec<Vec<u8>> = input
            .iter()
            .map(|i| i.iter().map(|(_, v)| *v).collect())
            .collect();

        while let Some(curr) = queue.pop_front() {
            let curr_val = input[curr.0][curr.1];

            if curr.1 < input[0].len() - 1
                && input[curr.0][curr.1 + 1] <= curr_val + 1
                && !visited.contains_key(&(curr.0, curr.1 + 1))
            {
                visited.insert((curr.0, curr.1 + 1), curr);
                queue.push_back((curr.0, curr.1 + 1));
            }

            if curr.0 < input.len() - 1
                && input[curr.0 + 1][curr.1] <= curr_val + 1
                && !visited.contains_key(&(curr.0 + 1, curr.1))
            {
                visited.insert((curr.0 + 1, curr.1), curr);
                queue.push_back((curr.0 + 1, curr.1));
            }

            if curr.1 > 0
                && input[curr.0][curr.1 - 1] <= curr_val + 1
                && !visited.contains_key(&(curr.0, curr.1 - 1))
            {
                visited.insert((curr.0, curr.1 - 1), curr);
                queue.push_back((curr.0, curr.1 - 1));
            }

            if curr.0 > 0
                && input[curr.0 - 1][curr.1] <= curr_val + 1
                && !visited.contains_key(&(curr.0 - 1, curr.1))
            {
                visited.insert((curr.0 - 1, curr.1), curr);
                queue.push_back((curr.0 - 1, curr.1));
            }
        }

        let mut answer_key = vec![vec![' '; input[0].len()]; input.len()];

        let mut path = end;
        let mut total = 0;

        if !visited.contains_key(&path) {
            continue;
        }

        while let Some(from) = visited.get(&path) {
            total += 1;

            let print = if from.0 > path.0 {
                '^'
            } else if from.0 < path.0 {
                'v'
            } else if from.1 > path.1 {
                '<'
            } else {
                '>'
            };
            answer_key[from.0][from.1] = print;

            if *from == start {
                break;
            }
            path = *from;
        }

        if total < min {
            min = total;
            answer_key[end.0][end.1] = 'E';
            answer_key[start.0][start.1] = 'S';
            shortest_print = answer_key.clone();
        }
    }

    println!(
        "{0}\n{1}\n{0}",
        (0..input[0].len() + 2).map(|_| '─').collect::<String>(),
        shortest_print
            .iter()
            .map(|line| format!("│{}│", line.iter().collect::<String>()))
            .collect::<Vec<String>>()
            .join("\n")
    );
    min
}
