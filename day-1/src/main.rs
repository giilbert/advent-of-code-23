use eyre::Result;
use nom::IResult;

type Output = u32;
type Input = Vec<String>;

const PART_1_EXPECTED_TEST_OUTPUT: Output = 142;
const PART_2_EXPECTED_TEST_OUTPUT: Output = 281;

/// Returns a vector of lines from the input.
fn parse_input(input: &str) -> IResult<&str, Input> {
    let lines = input.lines().map(|line| line.to_string()).collect();
    Ok(("", lines))
}

fn solve_part1(input: Input) -> Result<Output> {
    let mut accumulator = 0;

    for line in input {
        let letters = line
            .split("")
            .filter(|letter| letter.parse::<u32>().is_ok());

        let first_number = letters.clone().nth(0).unwrap().parse::<u32>().unwrap();
        let last_number = letters.clone().last().unwrap().parse::<u32>().unwrap();

        let sum = first_number * 10 + last_number;
        accumulator += sum;
    }

    Ok(accumulator)
}

fn part2_transform(line: &str) -> String {
    let mut new_line = line.to_string();

    for i in 0..line.len() {
        if line[i..].starts_with("one") {
            new_line.replace_range((i)..(i + 1), "1");
        } else if line[i..].starts_with("two") {
            new_line.replace_range((i)..(i + 1), "2");
        } else if line[i..].starts_with("three") {
            new_line.replace_range((i)..(i + 1), "3");
        } else if line[i..].starts_with("four") {
            new_line.replace_range((i)..(i + 1), "4");
        } else if line[i..].starts_with("five") {
            new_line.replace_range((i)..(i + 1), "5");
        } else if line[i..].starts_with("six") {
            new_line.replace_range((i)..(i + 1), "6");
        } else if line[i..].starts_with("seven") {
            new_line.replace_range((i)..(i + 1), "7");
        } else if line[i..].starts_with("eight") {
            new_line.replace_range((i)..(i + 1), "8");
        } else if line[i..].starts_with("nine") {
            new_line.replace_range((i)..(i + 1), "9");
        }
    }

    new_line
}

fn solve_part2(input: Input) -> Result<Output> {
    let transformed = input
        .iter()
        .map(|line| part2_transform(line))
        .collect::<Vec<_>>();

    solve_part1(transformed)
}

fn main() {
    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input).unwrap());

    let input = parse_input(include_str!("../real-input.txt")).unwrap().1;
    println!("Part 2: {:?}", solve_part2(input).unwrap());
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
            solve_part1(parse_input(include_str!("../test-input.txt")).unwrap().1).unwrap(),
            PART_1_EXPECTED_TEST_OUTPUT
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse_input(include_str!("../test-input-2.txt")).unwrap().1).unwrap(),
            PART_2_EXPECTED_TEST_OUTPUT
        );
    }
}
