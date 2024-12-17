use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(7);

fn is_possible(total: u64, factors: &mut VecDeque<u64>) -> bool {
    if factors.is_empty() {
        return false;
    }
    if factors.len() == 1 {
        return total == factors[0];
    }

    let first = factors.pop_back().expect("Checked");

    if first > total {
        return false;
    }

    if is_possible(total - first, &mut factors.clone()) {
        return true;
    }

    if total % first == 0 && is_possible(total / first, &mut factors.clone()) {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let new_line = line_ending::<&str, ()>;
    let data = separated_list1(
        new_line,
        separated_pair(u64, tag(": "), separated_list1(space1, u64)),
    )(input.trim())
    .expect("Correct input format")
    .1;

    let mut acc = 0u64;
    for (total, factors) in data {
        if is_possible(total, &mut VecDeque::from(factors)) {
            acc += total;
        }
    }
    Some(acc)
}

fn is_possible_part_two(total: u64, factors: &mut VecDeque<u64>) -> bool {
    if factors.is_empty() {
        return false;
    }
    if factors.len() == 1 {
        return total == factors[0];
    }

    let first = factors.pop_back().expect("Checked");

    if first > total {
        return false;
    }

    if is_possible_part_two(total - first, &mut factors.clone()) {
        return true;
    }

    if total % first == 0 && is_possible_part_two(total / first, &mut factors.clone()) {
        return true;
    }

    let a = total.to_string();
    let b = first.to_string();

    if let Ok(c) = (a[..a.len() - b.len()]).parse::<u64>() {
        if a.ends_with(&b) && is_possible_part_two(c, &mut factors.clone()) {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let new_line = line_ending::<&str, ()>;
    let data = separated_list1(
        new_line,
        separated_pair(u64, tag(": "), separated_list1(space1, u64)),
    )(input.trim())
    .expect("Correct input format")
    .1;

    let mut acc = 0u64;
    for (total, factors) in data {
        if is_possible_part_two(total, &mut VecDeque::from(factors)) {
            acc += total;
        }
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
