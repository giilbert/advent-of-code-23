use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::current,
};

use eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, digit1, newline},
    combinator::{map, map_res, opt},
    multi::{many0, many1, separated_list0},
    number,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    has_numbers: Vec<u32>,
}

type Output = u32;
type Input = Vec<Card>;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 13;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 30;

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list0(
        newline,
        map(
            tuple((
                tag("Card "),
                many0(tag(" ")),
                digit1,
                tag(": "),
                opt(many0(tag(" "))),
                separated_list0(many1(tag(" ")), map_res(digit1, str::parse::<u32>)),
                tag(" | "),
                opt(many0(tag(" "))),
                separated_list0(many1(tag(" ")), map_res(digit1, str::parse::<u32>)),
            )),
            |(_, _, _, _, _, winning_numbers, _, _, has_numbers)| Card {
                winning_numbers,
                has_numbers,
            },
        ),
    )(input)
}

fn count_overlaps(card: &Card) -> u32 {
    let winning_numbers = card.winning_numbers.iter().collect::<HashSet<_>>();
    let has_numbers = card.has_numbers.iter().collect::<HashSet<_>>();
    winning_numbers.intersection(&has_numbers).count() as u32
}

fn solve_part1(input: Input) -> Output {
    input
        .iter()
        .map(|card| {
            let number_of_overlaps = count_overlaps(&card);
            if number_of_overlaps == 0 {
                0
            } else {
                2u32.pow((number_of_overlaps as u32) - 1)
            }
        })
        .sum()
}

#[derive(Debug)]
struct QueuedCard {
    pub index: usize,
}

fn solve_part2(input: Input) -> Output {
    let mut queue: VecDeque<QueuedCard> = VecDeque::new();
    for index in 0..input.len() {
        queue.push_back(QueuedCard { index });
    }
    let mut count = 0;

    while let Some(QueuedCard {
        index: current_index,
    }) = queue.pop_back()
    {
        count += 1;
        let number_of_overlaps = count_overlaps(&input[current_index]);
        for i in 0usize..(number_of_overlaps as usize) {
            let next_card = QueuedCard {
                index: (1 + current_index + i),
            };
            queue.push_back(next_card);
        }
    }

    count
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
            solve_part2(parse_input(include_str!("../test-input-2.txt")).unwrap().1),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
