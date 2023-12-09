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

type Output = i64;
type Input = Vec<Vec<i64>>;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 114;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 2;

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list0(
        newline,
        separated_list1(
            tag(" "),
            map_res(
                tuple((opt(tag("-")), digit1)),
                |(negative, s): (Option<&str>, &str)| {
                    let mut s = s.to_string();
                    if let Some(negative) = negative {
                        s = format!("{}{}", negative, s);
                    }
                    s.parse::<i64>()
                },
            ),
        ),
    )(input)
}

fn calculate_differences(numbers: Vec<i64>) -> Vec<i64> {
    numbers.windows(2).map(|w| w[1] - w[0]).collect()
}

fn find_history(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut differences = calculate_differences(line.clone());
    let mut history = vec![line, differences.clone()];

    // while the differences are not all the same, keep
    // calculating and pushing to the history
    while !differences.iter().all(|&x| x == differences[0]) {
        differences = calculate_differences(differences);
        history.push(differences.clone());
    }

    history
}

fn solve_part1(input: Input) -> Output {
    input
        .into_iter()
        .map(|line| {
            find_history(line)
                .iter()
                .rev()
                .fold(0, |current_difference, line| {
                    line.last().expect("a last element") + current_difference
                })
        })
        .sum()
}

fn solve_part2(input: Input) -> Output {
    input
        .into_iter()
        .map(|line| {
            find_history(line)
                .iter()
                .rev()
                .fold(0, |current_difference, line| line[0] - current_difference)
        })
        .sum()
}

fn main() {
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input));

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
            solve_part2(parse_input(include_str!("../test-input.txt")).unwrap().1),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
