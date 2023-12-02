use eyre::Result;
use nom::{
    character::complete::{alpha0, alpha1, digit1, newline},
    combinator::map_res,
    multi::{many0, separated_list0},
    number,
    sequence::{delimited, tuple},
    IResult,
};

type Output = u32;
type Input = Vec<Game>;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

const PART_1_EXPECTED_TEST_OUTPUT: Output = 8;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 2286;

/// Returns a vector of lines from the input.
fn parse_input(input: &str) -> IResult<&str, Input> {
    use nom::bytes::complete::tag;

    let parse_hand = map_res(
        separated_list0(
            tag(", "),
            tuple((
                map_res(digit1, str::parse::<u32>),
                tag(" "),
                map_res(alpha1, |x| match x {
                    "red" => Ok::<_, ()>(Color::Red),
                    "green" => Ok(Color::Green),
                    "blue" => Ok(Color::Blue),
                    _ => panic!("Invalid color {x}"),
                }),
            )),
        ),
        |data| {
            let find = |color: Color| -> u32 {
                data.iter()
                    .find(|(_, _, c)| *c == color)
                    .unwrap_or(&(0, "", color))
                    .0
            };
            let red = find(Color::Red);
            let green = find(Color::Green);
            let blue = find(Color::Blue);
            Ok::<_, nom::Err<&str>>(Hand { red, green, blue })
        },
    );

    let parse_line = map_res(
        tuple((
            tag("Game "),
            map_res(digit1, str::parse::<u32>),
            tag(": "),
            separated_list0(tag("; "), parse_hand),
        )),
        |(_, id, _, hands)| Ok::<_, nom::Err<&str>>(Game { id, hands }),
    );

    separated_list0(tag("\n"), parse_line)(input)
}

fn solve_part1(input: Input) -> Output {
    input
        .iter()
        .filter(|game| {
            game.hands
                .iter()
                .all(|hand| hand.red <= 12 && hand.green <= 13 && hand.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

fn solve_part2(input: Input) -> Output {
    input
        .iter()
        .map(|game| {
            // find the max of each color
            game.hands.iter().fold((0, 0, 0), |acc, hand| {
                (
                    acc.0.max(hand.red),
                    acc.1.max(hand.green),
                    acc.2.max(hand.blue),
                )
            })
        })
        .map(|(r, g, b)| r * g * b)
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
            solve_part2(parse_input(include_str!("../test-input-2.txt")).unwrap().1),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
