use aoc_runner_derive::{aoc, aoc_generator};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use itertools::Itertools;
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
    prelude::*,
};
use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::{
        complete::{self, line_ending},
        is_alphabetic,
    },
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

type Input = HashMap<String, (i32, Vec<String>)>;

fn parse_line(input: &str) -> IResult<&str, (String, (i32, Vec<String>))> {
    tuple((
        preceded(tag("Valve "), Parser::into(take(2_usize))),
        tuple((
            preceded(tag(" has flow rate="), complete::i32),
            preceded(
                alt((
                    tag("; tunnel leads to valve "),
                    tag("; tunnels lead to valves "),
                )),
                separated_list1(tag(", "), Parser::into(take(2_usize))),
            ),
        )),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, list.into_iter().collect()))
}

#[aoc_generator(day16)]
pub fn day16_generator(input: &str) -> Input {
    let (input, output) = parse_input(input).unwrap();
    assert!(input.is_empty());
    output
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let graph = UnGraphMap::<&str, ()>::from_edges(
        input
            .iter()
            .flat_map(|(from, (_, to))| to.iter().map(|t| (from.as_str(), t.as_str()))),
    );

    let get_to_paths = input
        .iter()
        .filter_map(|(k, (cost, _))| if *cost > 0 { Some(k) } else { None })
        .collect_vec();
    let len_to = get_to_paths.len();

    let total: usize = (1..=len_to).product();

    get_to_paths
        .into_iter()
        .permutations(len_to)
        .par_bridge()
        .progress_count(total as u64)
        .map(|mut next_path| {
            let mut flow_rate = 0;
            let mut flow = 0;
            let mut minutes = 30;
            let mut curr = String::from("AA");
            let mut visited = vec![];

            while minutes > 0 {
                // 30 minute count-UP?

                let Some(next) = next_path.pop() else {
                    flow += minutes * flow_rate;
                    return flow;
                };
                visited.push(next);

                let path_map = dijkstra(&graph, &curr, Some(next.as_str()), |_| 1);
                let distance = path_map.get(next.as_str()).map(|v| *v).unwrap_or(minutes);

                let distance = distance + 1;
                if minutes < distance {
                    // couldn't reach node
                    continue;
                }

                minutes -= distance;
                flow += distance * flow_rate;
                flow_rate += input.get(next).map(|v| v.0).unwrap_or(0);

                curr = next.to_string();
            }

            flow
        })
        .max()
        .expect("No solutions")
}

#[aoc(day16, part2)]
pub fn solve_part2(_input: &Input) -> i64 {
    todo!();
}
