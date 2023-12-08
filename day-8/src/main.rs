use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::current,
    time::Instant,
};

use eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha0, alpha1, digit0, digit1, newline},
    combinator::{eof, map, map_res, opt},
    multi::{many0, many1, separated_list0, separated_list1},
    number,
    sequence::{delimited, tuple},
    AsChar, Err, IResult, Parser,
};
use rayon::iter::IntoParallelIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LeftRight {
    Left,
    Right,
}

type ThreeChars = [u8; 3];
fn string_to_three_chars(s: &str) -> ThreeChars {
    let mut chars = s.chars();
    [
        chars.next().unwrap() as u8,
        chars.next().unwrap() as u8,
        chars.next().unwrap() as u8,
    ]
}
const Z_AS_U8: u8 = 'Z' as u8;
#[derive(Debug)]
struct Node {
    pub name: ThreeChars,
    pub left: ThreeChars,
    pub right: ThreeChars,
}

type Output = u64;
#[derive(Debug)]
struct Input {
    sequence: Vec<LeftRight>,
    nodes: Vec<Node>,
}

const PART_1_EXPECTED_TEST_OUTPUT: Output = 6;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 6;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let parse_left = || map(tag("L"), |_| LeftRight::Left);
    let parse_right = || map(tag("R"), |_| LeftRight::Right);
    let parse_direction = || alt((parse_left(), parse_right()));

    let parse_three_letters = || map(take(3usize), |s: &str| s.to_string());

    map(
        tuple((
            many1(parse_direction()),
            tag("\n\n"),
            separated_list1(
                newline,
                map(
                    tuple((
                        parse_three_letters(),
                        tag(" = ("),
                        parse_three_letters(),
                        tag(", "),
                        parse_three_letters(),
                        tag(")"),
                    )),
                    |(name, _, left, _, right, _)| Node {
                        name: string_to_three_chars(&name),
                        left: string_to_three_chars(&left),
                        right: string_to_three_chars(&right),
                    },
                ),
            ),
            eof,
        )),
        |(sequence, _, nodes, _)| Input { sequence, nodes },
    )(input)
}

fn solve_part1(input: Input) -> Output {
    let nodes = input.nodes.iter().fold(HashMap::new(), |mut acc, node| {
        acc.insert(node.name.clone(), node);
        acc
    });

    let mut visited = 0;
    let mut current_node = ['A' as u8, 'A' as u8, 'A' as u8];

    for direction in input.sequence.iter().cloned().cycle() {
        let node = nodes.get(&current_node).unwrap();
        current_node = match direction {
            LeftRight::Left => node.left,
            LeftRight::Right => node.right,
        };

        visited += 1;

        if current_node[0] == Z_AS_U8 && current_node[1] == Z_AS_U8 && current_node[2] == Z_AS_U8 {
            break;
        }
    }

    visited
}

fn solve_part2(input: Input) -> Output {
    let nodes = input.nodes.iter().fold(HashMap::new(), |mut acc, node| {
        acc.insert(node.name.clone(), node);
        acc
    });

    let mut visited = 0;
    let mut current_nodes = input
        .nodes
        .iter()
        .filter(|node| node.name[2] == 'A' as u8)
        .map(|node| node.name.clone())
        .collect::<Vec<_>>();

    let distances = current_nodes.iter().map(|node| {
        let mut count = 0;
        let mut current = node;

        for direction in input.sequence.iter().cloned().cycle() {
            count += 1;
            let node = nodes.get(current).unwrap();
            current = match direction {
                LeftRight::Left => &node.left,
                LeftRight::Right => &node.right,
            };
            if current[2] == Z_AS_U8 {
                break;
            }
        }

        return count;
    });

    println!("current_nodes: {:?}", current_nodes.len());

    distances.fold(1, |acc, x| num::integer::lcm(acc, x))
}

fn main() {
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input));

    // lol
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 2: {:?}", solve_part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_input, solve_part1, solve_part2, PART_1_EXPECTED_TEST_OUTPUT,
        PART_2_EXPECTED_TEST_OUTPUT,
    };

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(parse_input(include_str!("../test-input.txt")).unwrap().1),
            PART_1_EXPECTED_TEST_OUTPUT
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse_input(include_str!("../test-input-2.txt")).unwrap().1),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
