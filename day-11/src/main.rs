use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::repeat,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Galaxy,
    Empty,
}

type Output = u32;
type Input = Vec<Vec<Cell>>;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 374;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 10;

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        tuple((
            separated_list1(
                newline,
                many0(map_res(take(1usize), |c| match c {
                    "." => Ok(Cell::Empty),
                    "#" => Ok(Cell::Galaxy),
                    _ => Err(()),
                })),
            ),
            eof,
        )),
        |(cells, ..)| cells,
    )(input)
}

fn manhattan_distance((y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> usize {
    (y1 as isize - y2 as isize).abs() as usize + (x1 as isize - x2 as isize).abs() as usize
}

fn solve_part1(input: Input) -> Output {
    let mut working_input = input.clone();

    let mut vertical_stretch_amount = 0;
    // vertical stretch
    for (row_index, line) in input.iter().enumerate() {
        // if the line is entirely empty, insert a new empty line at the same index
        if line.iter().all(|&c| c == Cell::Empty) {
            working_input.insert(
                row_index + vertical_stretch_amount,
                repeat(Cell::Empty).take(line.len()).collect(),
            );
            vertical_stretch_amount += 1;
        }
    }

    let mut horizontal_stretch_amount = 0;
    for column_index in 0..input[0].len() {
        // if the column is entirely empty, insert a new empty column at the same index
        if input.iter().all(|line| line[column_index] == Cell::Empty) {
            for line in working_input.iter_mut() {
                line.insert(column_index + horizontal_stretch_amount, Cell::Empty);
            }
            horizontal_stretch_amount += 1;
        }
    }

    // print the stretched map
    for line in working_input.iter_mut() {
        for cell in line.iter() {
            match cell {
                Cell::Galaxy => print!("#"),
                Cell::Empty => print!("."),
            }
        }
        print!("\n");
    }

    // (y, x)
    let all_galaxies = working_input
        .iter()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(column_index, cell)| match cell {
                    Cell::Galaxy => Some((row_index, column_index)),
                    Cell::Empty => None,
                })
        })
        .collect::<Vec<_>>();

    all_galaxies
        .iter()
        .map(|&galaxy_coordinate| {
            all_galaxies
                .iter()
                .filter(|&&coordinate| coordinate != galaxy_coordinate)
                .map(|coordinate| manhattan_distance(galaxy_coordinate, *coordinate))
                .sum::<usize>()
        })
        .sum::<usize>() as u32
        / 2
}

fn solve_part2(input: Input) -> Output {
    todo!();
}

fn main() {
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input));

    // let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    // println!("Part 2: {:?}", solve_part2(input));
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

    // #[test]
    // fn part2() {
    //     assert_eq!(
    //         solve_part2(parse_input(include_str!("../test-input-2.txt")).unwrap().1),
    //         PART_2_EXPECTED_TEST_OUTPUT
    //     );
    // }
}
