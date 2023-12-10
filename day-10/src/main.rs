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

type Output = u32;
type Input = Map;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 8;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    NS,     // |
    WE,     // -
    NE,     // L
    NW,     // J
    SW,     // 7
    SE,     // F
    Ground, // .
    Start,  // S
}

#[derive(Debug, Clone)]
struct Map {
    pub cells: Vec<Vec<Cell>>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        separated_list0(
            newline,
            many0(map_res(take(1usize), |s: &str| match s {
                "|" => Ok(Cell::NS),
                "-" => Ok(Cell::WE),
                "L" => Ok(Cell::NE),
                "J" => Ok(Cell::NW),
                "7" => Ok(Cell::SW),
                "F" => Ok(Cell::SE),
                "." => Ok(Cell::Ground),
                "S" => Ok(Cell::Start),
                _ => Err(()),
            })),
        ),
        |cells| Map { cells },
    )(input)
}

fn get_neighbors(cell: Cell, position: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    // println!("{:?} {:?}", cell, position);
    match cell {
        // X, Y
        Cell::NS => ((position.0, position.1 - 1), (position.0, position.1 + 1)),
        Cell::WE => ((position.0 - 1, position.1), (position.0 + 1, position.1)),
        Cell::NE => ((position.0, position.1 - 1), (position.0 + 1, position.1)),
        Cell::NW => ((position.0 - 1, position.1), (position.0, position.1 - 1)),
        Cell::SW => ((position.0 - 1, position.1), (position.0, position.1 + 1)),
        Cell::SE => ((position.0, position.1 + 1), (position.0 + 1, position.1)),
        _ => panic!("Invalid cell"),
    }
}

fn solve_part1(input: Input) -> Output {
    let starting_position = input
        .cells
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == Cell::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut queue = VecDeque::new();
    for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (x, y) = starting_position;
        let (x, y) = (x as isize + dx, y as isize + dy);
        if x < 0 || y < 0 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= input.cells[0].len() || y >= input.cells.len() {
            continue;
        }

        let cell = input.cells[y][x];
        if (cell == Cell::NS && (*dy == -1 || *dy == 1))
            || (cell == Cell::WE && (*dx == -1 || *dx == 1))
            || (cell == Cell::NW && (*dy == 1 || *dx == 1))
            || (cell == Cell::NE && (*dy == 1 || *dx == -1))
            || (cell == Cell::SW && (*dy == -1 || *dx == 1))
            || (cell == Cell::SE && (*dy == -1 || *dx == -1))
        {
            queue.push_back(((x, y), 1));
        }
    }

    let mut min_distances = HashMap::<(usize, usize), u32>::new();

    while let Some(to_search) = queue.pop_front() {
        let ((x, y), current_cell_distance) = to_search;
        let cell = input.cells[y][x];

        let (neighbor1, neighbor2) = get_neighbors(cell, (x, y));

        if neighbor1 != starting_position {
            if !min_distances
                .get(&neighbor1)
                .is_some_and(|d| *d < current_cell_distance)
            {
                min_distances.insert(neighbor1, current_cell_distance + 1);
                queue.push_back((neighbor1, current_cell_distance + 1));
            }
        }
        if neighbor2 != starting_position {
            if !min_distances
                .get(&neighbor2)
                .is_some_and(|d| *d < current_cell_distance)
            {
                min_distances.insert(neighbor2, current_cell_distance + 1);
                queue.push_back((neighbor2, current_cell_distance + 1));
            }
        }
    }

    min_distances.iter().map(|(_, v)| *v).max().unwrap()
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

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse_input(include_str!("../test-input.txt")).unwrap().1),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
