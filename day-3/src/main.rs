use std::{
    collections::{HashMap, HashSet},
    thread::current,
};

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
type Input = Vec<String>;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 4361;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 467835;

/// Returns a vector of lines from the input.
fn parse_input(input: &str) -> IResult<&str, Input> {
    Ok(("", input.lines().map(|s| s.to_string() + ".").collect()))
}

fn solve_part1(input: Input) -> Output {
    let is_valid_symbol = |c: char| !c.is_numeric() && c != '.';

    let is_engine_number = |start_index: usize, end_index: usize, current_line_index: usize| {
        let check_line = |line: &str| {
            line.chars()
                .skip(((start_index as i32) - 1).max(0) as usize)
                .take((end_index - start_index) + 3)
                .any(is_valid_symbol)
        };

        if current_line_index != 0 {
            if let Some(previous_line) = input.get(current_line_index - 1) {
                if check_line(previous_line) {
                    return true;
                }
            }
        }

        if let Some(next_line) = input.get(current_line_index + 1) {
            if check_line(next_line) {
                return true;
            }
        }

        if check_line(&input[current_line_index]) {
            return true;
        }

        false
    };

    let mut acc = 0;
    for (line_index, line) in input.iter().enumerate() {
        let mut num_start_index: Option<usize> = None;

        for (index, c) in line.chars().enumerate() {
            if c.is_numeric() && num_start_index.is_none() {
                num_start_index = Some(index);
            } else if !c.is_numeric() && num_start_index.is_some() {
                let start = num_start_index.unwrap();
                let end = index - 1;
                num_start_index = None;

                let num = line[start..=end].parse::<u32>().unwrap();
                if is_engine_number(start, end, line_index) {
                    acc += num;
                }
            }
        }
    }

    acc
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    start_index: usize,
    end_index: usize,
}

fn solve_part2(input: Input) -> Output {
    let mut numbers = HashMap::<usize, Vec<Number>>::new();

    for (line_index, line) in input.iter().enumerate() {
        let mut num_start_index: Option<usize> = None;

        for (index, c) in line.chars().enumerate() {
            if c.is_numeric() && num_start_index.is_none() {
                num_start_index = Some(index);
            } else if !c.is_numeric() && num_start_index.is_some() {
                let start = num_start_index.unwrap();
                let end = index - 1;
                num_start_index = None;

                let num = line[start..=end].parse::<u32>().unwrap();
                numbers.entry(line_index).or_default().push(Number {
                    value: num,
                    start_index: start,
                    end_index: end,
                });
            }
        }
    }

    let mut acc = 0;

    for (line_index, line) in input.iter().enumerate() {
        for (index, char) in line.chars().enumerate() {
            if char == '*' {
                // check its 8 neighbors
                const OFFSETS: &[(i32, i32)] = &[
                    (-1, -1), // top
                    (-1, 0),
                    (-1, 1),
                    (0, -1), // middle
                    (0, 1),
                    (1, -1), // bottom
                    (1, 0),
                    (1, 1),
                ];

                let mut adjacent_numbers: HashSet<Number> = HashSet::default();

                for (offset_line, offset_col) in OFFSETS {
                    let line_index = line_index as i32 + offset_line;
                    let col_index = index as i32 + offset_col;

                    if line_index >= 0 {
                        if let Some(line) = numbers.get(&(line_index as usize)) {
                            if let Some(num) = line.iter().find(|n| {
                                n.start_index as i32 <= col_index && n.end_index as i32 >= col_index
                            }) {
                                adjacent_numbers.insert(num.clone());
                            }
                        }
                    }
                }

                if adjacent_numbers.len() == 2 {
                    acc += adjacent_numbers
                        .iter()
                        .map(|n| n.value)
                        .fold(1, |a, b| a * b);
                }
            }
        }
    }

    acc
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
