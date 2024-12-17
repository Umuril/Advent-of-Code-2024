use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let new_line = line_ending::<&str, ()>;
    let result = separated_pair(
        separated_list1(new_line, separated_pair(u32, tag("|"), u32)),
        tag("\n\n"),
        separated_list1(new_line, separated_list1(tag(","), u32)),
    )(input);
    result.expect("Correct input format").1
}

fn find_rule(rules: &Vec<(u32, u32)>, pair: &[u32]) -> bool {
    for rule in rules {
        if rule.0 == pair[0] && rule.1 == pair[1] {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);
    let mut acc = 0;
    for update in &updates {
        if update.windows(2).all(|pair| find_rule(&rules, pair)) {
            let mid = update.len().div_euclid(2);
            acc += update.get(mid).unwrap();
        }
    }

    Some(acc as u64)
}

fn sort_by_rules(mut update: Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    loop {
        let mut has_updated = false;
        for i in 0..update.len() - 1 {
            if find_rule(rules, &[update[i + 1], update[i]]) {
                update.swap(i, i + 1);
                has_updated = true;
            }
        }
        if !has_updated {
            break;
        }
    }
    update
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let mut acc = 0;
    for mut update in updates {
        if update.windows(2).all(|pair| find_rule(&rules, pair)) {
            continue;
        }
        update = sort_by_rules(update, &rules);
        let mid = update.len().div_euclid(2);
        acc += update.get(mid).unwrap();
    }

    Some(acc as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
