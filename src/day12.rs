use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult,
};
use petgraph::{
    algo::{dijkstra, astar},
    prelude::{DiGraphMap, GraphMap},
    Directed,
};

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
        .expect("No start position");

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
        .expect("No start position");

    let input: Vec<Vec<u8>> = input
        .iter()
        .map(|row| row.iter().map(|v| v.1).collect())
        .collect();

    let mut edges = vec![];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let curr = input[i][j];

            if i > 0 && input[i - 1][j] <= curr + 1 {
                edges.push(((i, j), (i - 1, j)));
            }
            if j > 0 && input[i][j - 1] <= curr + 1 {
                edges.push(((i, j), (i, j - 1)));
            }
            if i < input.len() - 1 && input[i + 1][j] <= curr + 1 {
                edges.push(((i, j), (i + 1, j)));
            }
            if j < input[i].len() - 1 && input[i][j + 1] <= curr + 1 {
                edges.push(((i, j), (i, j + 1)));
            }
        }
    }

    // edges represents the movement each node can make
    let graph: GraphMap<(usize, usize), (), Directed> = DiGraphMap::from_edges(&edges);

    // compute from end to start
    let (len, path) =
        astar(&graph, start, |stop| stop == end, |_| 1, |_| 1).expect("No path from start to end");

    let mut answer_string = vec![vec![' '; input[0].len()]; input.len()];

    let start = path.first().unwrap();
    let last = path.last().unwrap();

    let mut from = start;
    for point in path.iter().skip(1) {
        answer_string[from.0][from.1] = if from.0 < point.0 {
            'v'
        } else if from.0 > point.0 {
            '^'
        } else if from.1 < point.1 {
            '>'
        } else {
            '<'
        };
        from = point;
    }
    answer_string[end.0][end.1] = 'E';
    answer_string[start.0][start.1] = 'S';

    answer_string[last.0][last.1] = 'E';
    answer_string[start.0][start.1] = 'S';

    println!(
        "{0}\n{1}\n{0}",
        (0..input[0].len()).map(|_| '🭺').collect::<String>(),
        answer_string
            .iter()
            .map(|l| format!("▍{}▍", l.iter().collect::<String>()))
            .collect::<Vec<String>>()
            .join("\n")
    );

    len
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> usize {
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

    let input: Vec<Vec<u8>> = input
        .iter()
        .map(|row| row.iter().map(|v| v.1).collect())
        .collect();

    let mut edges = vec![];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let curr = input[i][j];

            if i > 0 && input[i - 1][j] >= curr - 1 {
                edges.push(((i, j), (i - 1, j)));
            }
            if j > 0 && input[i][j - 1] >= curr - 1 {
                edges.push(((i, j), (i, j - 1)));
            }
            if i < input.len() - 1 && input[i + 1][j] >= curr - 1 {
                edges.push(((i, j), (i + 1, j)));
            }
            if j < input[i].len() - 1 && input[i][j + 1] >= curr - 1 {
                edges.push(((i, j), (i, j + 1)));
            }
        }
    }

    // edges represents the movement each node can make
    let graph: GraphMap<(usize, usize), (), Directed> = DiGraphMap::from_edges(&edges);

    let mut smallest = usize::MAX;

    let everywhere = dijkstra(&graph, end, None, |_| 1);
    for (index, len) in everywhere.iter() {
        let curr = input[index.0][index.1];
        if curr == 0 && len < &smallest {
            smallest = *len;
        }
    }
    smallest
}
