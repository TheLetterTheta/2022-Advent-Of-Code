use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::{
    algo::dijkstra,
    prelude::*,
};
use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

type Input = HashMap<String, (u32, Vec<String>)>;

fn parse_line(input: &str) -> IResult<&str, (String, (u32, Vec<String>))> {
    tuple((
        preceded(tag("Valve "), Parser::into(take(2_usize))),
        tuple((
            preceded(tag(" has flow rate="), complete::u32),
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
    let _input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
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
pub fn solve_part1(input: &Input) -> u32 {
    let mut graph = UnGraphMap::<&str, u32>::from_edges(
        input
            .iter()
            .flat_map(|(from, (_flow, to))| to.iter().map(|t| (from.as_str(), t.as_str(), 1))),
    );

    let mut new_edges = vec![];

    // Remove nodes with 0 flow
    for (node, _) in input
        .iter()
        .filter(|&(id, (flow, _))| *flow == 0 && !id.eq(&"AA"))
    {
        let edges = graph.edges(node).collect_vec();
        for (from, to, distance) in edges.iter().enumerate().flat_map(|(index, edge)| {
            edges
                .iter()
                .skip(index + 1)
                .map(|other| (edge.1, other.1, edge.2 + other.2))
        }) {
            new_edges.push((from, to, distance));
        }

        for edge in new_edges.drain(..) {
            graph.add_edge(edge.0, edge.1, edge.2);
        }

        graph.remove_node(node);
    }

    // Graphviz format
    // println!("{:?}", Dot::with_config(&graph, &[]));

    let input: HashMap<&str, u32> = input
        .iter()
        .filter_map(|(id, &(flow, _))| {
            if id.eq(&"AA") || flow > 0 {
                Some((id.as_str(), flow))
            } else {
                None
            }
        })
        .collect();

    let shortest_paths = input
        .keys()
        .flat_map(|node| {
            dijkstra(&graph, node, None, |w| *w.weight())
                .into_iter()
                .filter(|&(_,dist)| dist > 0)
                .map(move |(to, dist)| ((*node, to), dist))
        })
        .collect::<HashMap<(&str, &str), u32>>();

    // Node, minutes, flow_rate, elapsed_flow, open valves
    let mut path = vec![(
        "AA",
        30,
        0,
        0,
        ["AA"].into_iter().collect::<HashSet<&str>>(),
    )];
    let mut max = 0;

    while let Some((curr, minutes, flow, elapsed, opened)) = path.pop() {
        let mut at_end = true;

        for (edge, distance, next_flow) in shortest_paths
            .iter()
            .filter(|((from, to), _)| from == &curr && !opened.contains(to))
            .filter_map(|((_, edge), &distance)| {
                if let Some(flow) = input.get(edge) && distance < minutes - 1 {
                    Some((edge, distance, flow))
                } else {
                    None
                }
            })
            .sorted_unstable_by_key(|&(_, distance, next_flow)| next_flow * (minutes - distance))
        {
            let mut open_next = opened.clone();
            open_next.insert(edge);
            path.push((
                edge,
                (minutes - distance) - 1,
                flow + next_flow,
                // NEED TO ACCOUNT FOR MINUTE TURNING IT ON
                elapsed + (flow * (1 + distance)),
                open_next,
            ));
            at_end = false;
        }

        if at_end {
            // every valve is on?
            let curr_max = elapsed + (minutes * flow);
            if curr_max > max {
                max = curr_max;
            }
        }
    }

    max
}

#[aoc(day16, part2)]
pub fn solve_part2(_input: &Input) -> i64 {
    todo!();
}
