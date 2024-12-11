use std::collections::HashMap;

use nom::{character::complete::space1, character::complete::u32, multi::separated_list1};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let data = separated_list1(space1::<&str, ()>, u32)(input)
        .expect("Correct input format")
        .1;

    Some(solve(25, data))
}

fn next_num(x: u128) -> Vec<u128> {
    if x == 0 {
        vec![1]
    } else if x.to_string().len() % 2 == 0 {
        let z = x.to_string();
        let (a, b) = z.split_at(x.to_string().len() / 2);
        vec![a.parse::<u128>().unwrap(), b.parse::<u128>().unwrap()]
    } else {
        vec![x * 2024]
    }
}

fn solve(blinks: u32, data: Vec<u32>) -> u64 {
    let mut map: HashMap<u128, u64> = HashMap::new();
    for num in data {
        *map.entry(num as u128).or_default() += 1;
    }

    for _ in 0..blinks {
        let mut new_map: HashMap<u128, u64> = HashMap::new();
        for (k, v) in map {
            let values = next_num(k);
            for val in values {
                new_map.entry(val).and_modify(|n| *n += v).or_insert(v);
            }
        }
        map = new_map;
    }

    map.values().sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = separated_list1(space1::<&str, ()>, u32)(input)
        .expect("Correct input format")
        .1;

    Some(solve(75, data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
