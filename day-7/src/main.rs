use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::current,
    time::Instant,
};

use eyre::Result;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha0, alpha1, digit0, digit1, newline},
    combinator::{eof, map, map_res, opt},
    multi::{many0, many1, separated_list0, separated_list1},
    number,
    sequence::{delimited, tuple},
    AsChar, Err, IResult, Parser,
};
use rayon::iter::IntoParallelIterator;

type Output = u64;
type Input = Vec<Game>;

#[derive(Debug)]
struct Game {
    hand: Vec<Card>,
    bid: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
#[repr(u8)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from_char(letter: char) -> Result<Self, ()> {
        match letter {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

const PART_1_EXPECTED_TEST_OUTPUT: Output = 6440;

fn parse_input(input: &str) -> IResult<&str, Input> {
    tuple((
        separated_list1(
            newline,
            map(
                tuple((
                    many0(map_res(take(1usize), |s: &str| {
                        Card::from_char(s.chars().next().unwrap())
                    })),
                    tag(" "),
                    map_res(digit1, str::parse::<u64>),
                )),
                |(cards, _, bid)| Game { hand: cards, bid },
            ),
        ),
        eof,
    ))(input)
    .map(|(rest, (games, _))| (rest, games))
}

fn get_type(hand: &[Card]) -> u64 {
    let counter = hand.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });

    // five of a kind
    if counter.iter().any(|(_, &count)| count == 5) {
        return 6;
    }

    // four of a kind
    if counter.iter().any(|(_, &count)| count == 4) {
        return 5;
    }

    // full house
    let frequencies = counter.values().collect::<HashSet<_>>();
    if frequencies.len() == 2 && frequencies.contains(&2) && frequencies.contains(&3) {
        return 4;
    }

    // three of a kind
    if counter.iter().any(|(_, &count)| count == 3) {
        return 3;
    }

    // two pair
    if counter.iter().filter(|(_, &count)| count == 2).count() == 2 {
        return 2;
    }

    // there are 2 duplicate cards (one pair)
    if counter.iter().any(|(_, &count)| count == 2) {
        return 1;
    }

    // there are no duplicate cards (weakest hand)
    if counter.iter().all(|(_, &count)| count == 1) {
        return 0;
    }

    panic!("what")
}

fn solve_part1(input: Input) -> Output {
    let mut a = input
        .into_iter()
        .map(|game| (get_type(&game.hand), game))
        .collect::<Vec<_>>();

    a.sort_by(|a, b| {
        // if the types are different, the bigger type wins
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            // if the types arent the same, the higher hand wins

            for (a, b) in a.1.hand.iter().zip(b.1.hand.iter()) {
                if a > b {
                    return std::cmp::Ordering::Greater;
                } else if a < b {
                    return std::cmp::Ordering::Less;
                }
            }

            return std::cmp::Ordering::Equal;
        }
    });

    a.into_iter()
        .enumerate()
        .map(|(i, (_, game))| {
            println!("{}: {:?}", i, game);
            (i, game)
        })
        .fold(0, |acc, (rank, game)| acc + (rank as u64 + 1) * game.bid)
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
