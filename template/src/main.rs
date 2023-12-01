use eyre::Result;
use nom::IResult;

type Output = ();
struct Input {}

const EXPECTED_TEST_OUTPUT: Output = ();

fn parse_input(input: &str) -> IResult<&str, Input> {
    todo!()
}

fn solve_part1(input: Input) -> Result<Output> {
    todo!()
}

fn solve_part2(input: Input) -> Result<Output> {
    todo!()
}

fn main() {
    let input = parse_input(include_str!("../part-1-input.txt")).unwrap().1;
    println!("Part 1: {:?}", solve_part1(input).unwrap());

    // let input = parse_input(include_str!("../part-1-input.txt")).unwrap().1;
    // println!("Part 1: {:?}", solve(input).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part1, EXPECTED_TEST_OUTPUT};

    #[test]
    fn it_works() {
        assert_eq!(
            solve_part1(parse_input(include_str!("../test-input.txt")).unwrap().1).unwrap(),
            EXPECTED_TEST_OUTPUT
        );
    }
}
