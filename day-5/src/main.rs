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
#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<Map>,
    soil_to_fertilizer_map: Vec<Map>,
    fertilizer_to_water_map: Vec<Map>,
    water_to_light_map: Vec<Map>,
    light_to_temperature_map: Vec<Map>,
    temperature_to_humidity_map: Vec<Map>,
    humidity_to_location_map: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    // first number
    destination_range_start: u64,
    // second number
    source_range_start: u64,
    // third number
    range_length: u64,
}

fn resolve_maps(maps: &[Map], x: u64) -> u64 {
    for map in maps {
        if x >= map.source_range_start && map.source_range_start + map.range_length > x {
            return x - map.source_range_start + map.destination_range_start;
        }
    }
    return x;
}

const PART_1_EXPECTED_TEST_OUTPUT: Output = 35;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 46;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let space_separated_numbers = || separated_list1(tag(" "), map_res(digit1, str::parse::<u64>));
    let nom_map = || {
        separated_list1(
            newline,
            map(space_separated_numbers(), |numbers| Map {
                destination_range_start: numbers[0],
                source_range_start: numbers[1],
                range_length: numbers[2],
            }),
        )
    };

    map(
        tuple((
            tag("seeds: "),
            space_separated_numbers(),
            tag("\n\nseed-to-soil map:\n"),
            nom_map(),
            tag("\n\nsoil-to-fertilizer map:\n"),
            nom_map(),
            tag("\n\nfertilizer-to-water map:\n"),
            nom_map(),
            tag("\n\nwater-to-light map:\n"),
            nom_map(),
            tag("\n\nlight-to-temperature map:\n"),
            nom_map(),
            tag("\n\ntemperature-to-humidity map:\n"),
            nom_map(),
            tag("\n\nhumidity-to-location map:\n"),
            nom_map(),
            eof,
        )),
        |(
            _,
            seeds,
            _,
            seed_to_soil_map,
            _,
            soil_to_fertilizer_map,
            _,
            fertilizer_to_water_map,
            _,
            water_to_light_map,
            _,
            light_to_temperature_map,
            _,
            temperature_to_humidity_map,
            _,
            humidity_to_location_map,
            _,
        )| Input {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    )(input)
}

fn solve_part1(input: Input) -> Output {
    input
        .seeds
        .iter()
        .map(|seed| {
            let a = resolve_maps(&input.seed_to_soil_map, *seed);
            let b = resolve_maps(&input.soil_to_fertilizer_map, a);
            let c = resolve_maps(&input.fertilizer_to_water_map, b);
            let d = resolve_maps(&input.water_to_light_map, c);
            let e = resolve_maps(&input.light_to_temperature_map, d);
            let f = resolve_maps(&input.temperature_to_humidity_map, e);
            let g = resolve_maps(&input.humidity_to_location_map, f);
            g
        })
        .min()
        .unwrap()
}

fn solve_part2(input: Input) -> Output {
    input
        .seeds
        .chunks(2)
        .map(|chunk| {
            use rayon::iter::ParallelIterator;

            let start = chunk[0];
            let count = chunk[1];

            let ret = (start..(start + count))
                .into_par_iter()
                .map(|seed| {
                    let a = resolve_maps(&input.seed_to_soil_map, seed);
                    let b = resolve_maps(&input.soil_to_fertilizer_map, a);
                    let c = resolve_maps(&input.fertilizer_to_water_map, b);
                    let d = resolve_maps(&input.water_to_light_map, c);
                    let e = resolve_maps(&input.light_to_temperature_map, d);
                    let f = resolve_maps(&input.temperature_to_humidity_map, e);
                    let g = resolve_maps(&input.humidity_to_location_map, f);
                    g
                })
                .min()
                .unwrap();

            ret
        })
        .min()
        .unwrap()
}

fn main() {
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input));

    let now = Instant::now();
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 2: {:?}", solve_part2(input));
    println!("Total: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_input, resolve_maps, solve_part1, solve_part2, Map, PART_1_EXPECTED_TEST_OUTPUT,
        PART_2_EXPECTED_TEST_OUTPUT,
    };

    #[test]
    fn map_resolve() {
        let maps = vec![
            Map {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            Map {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ];

        assert_eq!(resolve_maps(&maps, 0), 0);
        assert_eq!(resolve_maps(&maps, 1), 1);
        assert_eq!(resolve_maps(&maps, 48), 48);
        assert_eq!(resolve_maps(&maps, 50), 52);
        assert_eq!(resolve_maps(&maps, 51), 53);
        assert_eq!(resolve_maps(&maps, 96), 98);
        assert_eq!(resolve_maps(&maps, 97), 99);
        assert_eq!(resolve_maps(&maps, 98), 50);
        assert_eq!(resolve_maps(&maps, 99), 51);
    }

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
