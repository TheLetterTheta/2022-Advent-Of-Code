use aoc_runner_derive::{aoc, aoc_generator};
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
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
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

    let mut visited: HashSet<&str> = ["AA"].into();
    let mut flow_rate = 0;
    let mut flow = 0;
    let mut curr = "AA";
    let mut minutes = 30;

    while minutes > 0 {
        // 30 minute count-UP?

        let find_next = dijkstra(&graph, curr, None, |_| 1);
        let (next, distance, cost) = find_next
            .iter()
            .filter_map(|(node, distance)| {
                if visited.contains(node) || distance >= &minutes {
                    None
                } else {
                    Some((
                        node,
                        *distance + 1,
                        input
                            .get(node.to_owned())
                            .map(|v| v.0 * (minutes - (1 + distance)))?
                            .checked_sub((1 + distance) * flow_rate)?,
                    ))
                }
            })
            .max_by_key(|(_, _, cost)| *cost)
            .unwrap_or((&"", minutes, 0));

        minutes -= distance;
        flow += distance * flow_rate;
        flow_rate += input.get(next.to_owned()).map(|v| v.0).unwrap_or(0);
        visited.insert(next);
        curr = next;
        println!("Traveled for {} minutes (time remaining {}),to {} with {} flow rate. Turned on flow rate of {}. Current flow {}.",
        &distance, minutes,&next, &flow_rate,  input.get(next.to_owned()).map(|v| v.0).unwrap_or(0), &flow);
    }

    flow
}

#[aoc(day16, part2)]
pub fn solve_part2(_input: &Input) -> i64 {
    todo!();
}
