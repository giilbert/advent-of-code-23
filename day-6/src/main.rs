use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::current,
    time::Instant,
};

use eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, digit0, digit1, newline},
    combinator::{eof, map, map_res, opt},
    multi::{many0, many1, separated_list0, separated_list1},
    number,
    sequence::{delimited, tuple},
    Err, IResult, Parser,
};
use rayon::iter::IntoParallelIterator;

type Output = u64;
type Input = Vec<Race>;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

const PART_1_EXPECTED_TEST_OUTPUT: Output = 288;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let space_separated_numbers =
        || separated_list1(many1(tag(" ")), map_res(digit1, str::parse::<u64>));

    map(
        tuple((
            tuple((tag("Time: "), many0(tag(" ")))),
            space_separated_numbers(),
            tuple((tag("\nDistance: "), many0(tag(" ")))),
            space_separated_numbers(),
            eof,
        )),
        |(_, times, _, distances, _)| {
            times
                .iter()
                .cloned()
                .zip(distances)
                .map(|(time, distance)| Race {
                    time,
                    record: distance,
                })
                .collect()
        },
    )(input)
}

fn solve_part1(input: Input) -> Output {
    input
        .into_iter()
        .map(|race| {
            (1..(race.time))
                .filter(|hold_time| {
                    let speed = hold_time;
                    let time = race.time - hold_time;
                    let distance = speed * time;
                    distance > race.record
                })
                .count() as u64
        })
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input));

    // lol
    let input = parse_input(include_str!("../real-input-2.txt")).unwrap().1;
    println!("Part 2: {:?}", solve_part1(input));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part1, PART_1_EXPECTED_TEST_OUTPUT};

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(parse_input(include_str!("../test-input.txt")).unwrap().1),
            PART_1_EXPECTED_TEST_OUTPUT
        );
    }
}
