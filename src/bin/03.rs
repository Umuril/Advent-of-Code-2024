use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    combinator::{map, map_res, value},
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(3);

fn find_num3(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(1, 3, char::is_numeric), |s: &str| {
        s.parse::<u32>()
    })(input)
}

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(find_num3, tag(","), find_num3),
        tag(")"),
    )(input)
}

fn parse_part_one(input: &str) -> Vec<(u32, u32)> {
    let x = many1(alt((parse_mul, value((0, 0), take(1u32)))))(input);
    x.expect("Correct input format").1
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_part_one(input);
    Some(
        data.iter()
            .filter(|(x, y)| *x < 1000 && *y < 1000)
            .map(|(x, y)| *x * *y)
            .sum(),
    )
}

struct State {
    state: bool,
    pairs: Vec<(u32, u32)>,
}

impl State {
    fn new() -> Self {
        Self {
            state: true,
            pairs: Vec::new(),
        }
    }
}

fn parse_part_two(input: &str) -> State {
    let x = fold_many1(
        alt((
            value((Some(true), (0, 0)), tag("do()")),
            value((Some(false), (0, 0)), tag("don't()")),
            map(parse_mul, |pair| (None, pair)),
            value((None, (0, 0)), take(1u32)),
        )),
        State::new,
        |mut acc: State, item| {
            if let Some(new_state) = item.0 {
                acc.state = new_state;
            }
            if acc.state && item.1 .0 > 0 {
                acc.pairs.push(item.1);
            }
            acc
        },
    )(input);

    x.expect("Correct input format").1
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_part_two(input);
    Some(
        data.pairs
            .iter()
            .filter(|(x, y)| *x < 1000 && *y < 1000)
            .map(|(x, y)| *x * *y)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
