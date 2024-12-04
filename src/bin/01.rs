use std::{collections::HashMap, iter::zip};

use nom::{
    character::complete::{line_ending, space1, u32},
    combinator::opt,
    multi::fold_many0,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(1);

fn parse_part_one(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    fold_many0(
        terminated(separated_pair(u32, space1, u32), opt(line_ending)),
        || (Vec::new(), Vec::new()),
        |mut acc: (Vec<u32>, Vec<u32>), item: (u32, u32)| {
            acc.0.push(item.0);
            acc.1.push(item.1);
            acc
        },
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_part_one(input).ok().map(|(_, (mut v1, mut v2))| {
        v1.sort();
        v2.sort();
        zip(v1, v2).map(|(x, y)| x.abs_diff(y)).sum()
    })
}

fn parse_part_two(input: &str) -> IResult<&str, (Vec<u32>, HashMap<u32, u32>)> {
    fold_many0(
        terminated(separated_pair(u32, space1, u32), opt(line_ending)),
        || (Vec::new(), HashMap::new()),
        |mut acc: (Vec<u32>, HashMap<u32, u32>), item: (u32, u32)| {
            acc.0.push(item.0);
            *acc.1.entry(item.1).or_insert(0) += 1;
            acc
        },
    )(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    parse_part_two(input)
        .ok()
        .map(|(_, (v1, v2))| v1.iter().map(|x| x * *v2.get(x).unwrap_or(&0u32)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
