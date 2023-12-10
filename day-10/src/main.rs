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

type Output = u32;
type Input = Map;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 8;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    NS, // |
    WE, // -
    NE, // L
    NW, // J
    SW, // 7
    SE, // F
    Gr, // .
    St, // S

    IG, // introduced ground (" ")
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
                "." => Ok(Cell::Gr),
                "S" => Ok(Cell::St),
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
                if *cell == Cell::St {
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

#[rustfmt::skip]
fn enlarge_cell(cell: Cell) -> &'static [Cell; 9] {
    use Cell as C;
    match cell {
        C::St => &[
            C::IG, C::NS, C::IG,
            C::WE, C::St, C::WE,
            C::IG, C::NS, C::IG,
        ],
        C::NS => &[
            C::IG, C::NS, C::IG,
            C::IG, C::NS, C::IG,
            C::IG, C::NS, C::IG,
        ],
        C::WE => &[
            C::IG, C::IG, C::IG,
            C::WE, C::WE, C::WE,
            C::IG, C::IG, C::IG,
        ],
        C::NE => &[
            C::IG, C::NS, C::IG,
            C::IG, C::NE, C::WE,
            C::IG, C::IG, C::IG,
        ],
        C::NW => &[
            C::IG, C::NS, C::IG,
            C::WE, C::NW, C::IG,
            C::IG, C::IG, C::IG,
        ],
        C::SW => &[
            C::IG, C::IG, C::IG,
            C::WE, C::SW, C::IG,
            C::IG, C::NS, C::IG,
        ],
        C::SE => &[
            C::IG, C::IG, C::IG,
            C::IG, C::SE, C::WE,
            C::IG, C::NS, C::IG,
        ],
        C::Gr => &[
            C::Gr, C::Gr, C::Gr,
            C::Gr, C::Gr, C::Gr,
            C::Gr, C::Gr, C::Gr,
        ],
        C::IG => panic!("Invalid cell")
    }
}

fn find_main_loop(
    starting_position: (usize, usize),
    cells: &Vec<Vec<Cell>>,
) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(starting_position);

    for (dx, dy) in &[(3, 0), (-3, 0), (0, 3), (0, -3)] {
        let (x, y) = starting_position;
        let (x, y) = (x as isize + dx, y as isize + dy);
        if x < 0 || y < 0 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= cells[0].len() || y >= cells.len() {
            continue;
        }

        // this (x, y) is the node next to the starting position in the untransformed map
        let cell = cells[y][x];
        if (cell == Cell::NS && (*dy == -3 || *dy == 3))
            || (cell == Cell::WE && (*dx == -3 || *dx == 3))
            || (cell == Cell::NW && (*dy == 3 || *dx == 3))
            || (cell == Cell::NE && (*dy == 3 || *dx == -3))
            || (cell == Cell::SW && (*dy == -3 || *dx == 3))
            || (cell == Cell::SE && (*dy == -3 || *dx == -3))
        {
            queue.push_back((
                (starting_position.0 as isize + dx / 3) as usize,
                (starting_position.1 as isize + dy / 3) as usize,
            ));
        }
    }

    while let Some(to_search) = queue.pop_back() {
        let (x, y) = to_search;
        // println!("to_search: {:?}", to_search);
        let cell = cells[y][x];

        let (neighbor1, neighbor2) = get_neighbors(cell, (x, y));

        if !visited.contains(&neighbor1) {
            visited.insert(neighbor1);
            queue.push_back(neighbor1);
        }
        if !visited.contains(&neighbor2) {
            visited.insert(neighbor2);
            queue.push_back(neighbor2);
        }
    }

    println!("visited {} cells", visited.len());

    visited
}

fn solve_part2(input: Input) -> Output {
    let mut bigger_cells = input
        .cells
        .clone()
        .into_iter()
        .map(|line| {
            (0..3)
                .map(|y| {
                    line.iter()
                        .map(|&cell| *enlarge_cell(cell))
                        .map(|cell| {
                            // each chunk is a row
                            cell.chunks(3)
                                .map(|chunk| chunk.to_vec())
                                .collect::<Vec<_>>()
                        })
                        .map(|row| row[y].clone())
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
        .chunks(3 * input.cells[0].len())
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    let starting_position = bigger_cells
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == Cell::St {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    // remove stragglers from starting position
    for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (x, y) = starting_position;
        let (x, y) = (x as isize + dx, y as isize + dy);
        if x < 0 || y < 0 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= bigger_cells[0].len() || y >= bigger_cells.len() {
            continue;
        }

        let (next_x, next_y) = (x as isize + dx, y as isize + dy);

        // check if next_x and next_y are out of bounds
        if next_x < 0 || next_y < 0 {
            bigger_cells[y][x] = Cell::IG;
            continue;
        }
        let (next_x, next_y) = (next_x as usize, next_y as usize);
        if next_x >= bigger_cells[0].len() || next_y >= bigger_cells.len() {
            bigger_cells[y][x] = Cell::IG;
            continue;
        }
        let next_cell = bigger_cells[next_y][next_x];

        if !((*dx == 1 || *dx == -1) && next_cell == Cell::WE
            || (*dy == 1 || *dy == -1) && next_cell == Cell::NS)
        {
            bigger_cells[y][x] = Cell::IG;
        }
    }

    for line in bigger_cells.iter() {
        for cell in line.iter() {
            print!(
                "{}",
                match cell {
                    Cell::NS => "|",
                    Cell::WE => "-",
                    Cell::NE => "L",
                    Cell::NW => "J",
                    Cell::SW => "7",
                    Cell::SE => "F",
                    Cell::Gr => ".",
                    Cell::St => "S",
                    Cell::IG => " ",
                }
            );
        }
        print!("\n");
    }

    println!("starting position = {:?}", starting_position);

    let main_loop_cells = find_main_loop(starting_position, &bigger_cells);

    let x_length = bigger_cells[0].len();
    let y_length = bigger_cells.len();
    println!("x_length = {:?}", x_length);
    println!("y_length = {:?}", y_length);
    // iterator over all the edge cells of the map
    let edges =
        // top edge
        (repeat(0).zip(0..y_length))
        // bottom edge
        .chain(repeat(x_length - 1).zip(0..y_length))
        // left edge
        .chain((0..x_length).zip(repeat(0)))
        // right edge
        .chain((0..x_length).zip(repeat(y_length - 1))).collect::<Vec<_>>();

    println!("edges = {:?}", edges.len());

    let mut outside_visited = HashSet::<(usize, usize)>::new();
    let mut outside_queue = VecDeque::with_capacity(1024);
    let mut outside_introduced_ground_map = HashSet::<(usize, usize)>::new();
    let mut outside_introduced_ground_count = 0;

    for (x, y) in edges {
        outside_queue.push_back((x, y));
    }

    while let Some((x, y)) = outside_queue.pop_back() {
        if outside_visited.contains(&(x, y)) {
            continue;
        }
        if !main_loop_cells.contains(&(x, y)) {
            outside_visited.insert((x, y));
        }

        if bigger_cells[y][x] == Cell::IG {
            outside_introduced_ground_map.insert((x, y));
            outside_introduced_ground_count += 1;
        }

        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (x, y) = (x as isize + dx, y as isize + dy);
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if x >= x_length || y >= y_length {
                continue;
            }

            if !outside_visited.contains(&(x, y)) && !main_loop_cells.contains(&(x, y)) {
                outside_queue.push_back((x, y));
            }
        }
    }

    println!("---------------------------------");

    let outside_visited_count = outside_visited.len();
    let outside_cells = (outside_visited_count - outside_introduced_ground_count) / 9;

    println!(
        "a > outside_introduced_ground_count = {}",
        outside_introduced_ground_count
    );
    println!("  > outside_cells = {}", outside_cells);

    let actual_area = input.cells.len() * input.cells[0].len();
    println!("b > actual_area = {}", actual_area);

    let inside = actual_area - outside_cells - main_loop_cells.len() / 3;
    println!("  > inside = {}", inside);

    // let mut outside_cells: Vec<Vec<bool>> = vec![vec![false; x_length]; y_length];
    // for (x, y) in outside_visited {
    //     outside_cells[y][x] = true;
    // }
    // for line in outside_cells.iter() {
    //     for cell in line.iter() {
    //         print!("{}", if *cell { "X" } else { " " });
    //     }
    //     print!("\n");
    // }

    inside as u32
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
            solve_part2(parse_input(include_str!("../test-input-2.txt")).unwrap().1),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
