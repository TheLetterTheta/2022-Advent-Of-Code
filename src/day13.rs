use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element {
    Digit(u16),
    List(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Element::Digit(left_num) => match other {
                Element::Digit(right_num) => left_num.cmp(right_num),
                Element::List(_) => {
                    Element::cmp(&Element::List(vec![Element::Digit(*left_num)]), other)
                }
            },
            Element::List(left_list) => match other {
                Element::Digit(right_num) => {
                    Element::cmp(self, &Element::List(vec![Element::Digit(*right_num)]))
                }
                Element::List(right_list) => left_list.cmp(right_list),
            },
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Input = Vec<(Vec<Element>, Vec<Element>)>;

fn parse_list(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(
        tag("["),
        separated_list0(
            tag(","),
            alt((
                map(complete::u16, Element::Digit),
                map(parse_list, Element::List),
            )),
        ),
        tag("]"),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(
        tuple((line_ending, line_ending)),
        separated_pair(parse_list, line_ending, parse_list),
    )(input)
}

#[aoc_generator(day13)]
pub fn day13_generator(input: &str) -> Input {
    let (remainder, input) = parse_input(input).unwrap();
    assert!(remainder.is_empty());
    input
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.cmp(right) == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut combined = input
        .iter()
        .flat_map(|(left, right)| [left.clone(), right.clone()])
        .collect::<Vec<_>>();

    let two = vec![Element::List(vec![Element::Digit(2)])];
    let six = vec![Element::List(vec![Element::Digit(6)])];

    combined.push(two.clone());
    combined.push(six.clone());

    combined.sort_unstable();

    combined
        .into_iter()
        .enumerate()
        .filter_map(|(index, list)| {
            if list == two || list == six {
                Some(index + 1)
            } else {
                None
            }
        })
        .product()
}
