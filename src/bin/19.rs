use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(19);


fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, patterns) = separated_list1(tag(", "), alpha1)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, designs) = separated_list1(tag("\n"), alpha1)(input)?;

    Ok((input, (patterns, designs)))
}

fn check(patterns: &Vec<&str>, design: &str) -> bool {
    let mut queue = HashSet::new();
    queue.insert(design);

    while let Some(d) = queue.iter().next().cloned() {
        queue.remove(&d);
        // println!("{:?}", queue);
        if d.len() == 0 {
            return true;
        }
        // println!("Trying: {d}");
        for pattern in patterns {
            if d.starts_with(pattern) {
                queue.insert(&d[pattern.len()..]);
            }
        }
    }


    false
}

pub fn part_one(input: &str) -> Option<String> {
    let (patterns, designs) = parse_input(input).expect("Correct input format").1;

    let mut acc = 0;
    for design in designs {
        if check(&patterns, design) {
            acc += 1;
        }
    }

    Some(acc.to_string())
}

fn count(patterns: &Vec<&str>, design: &str) -> u64 {
    let mut queue = HashMap::new();
    queue.insert(design, 1);
    let mut acc = 0;

    loop {
        if queue.len() == 0 {
            break
        }
        let d = queue.clone().into_keys().next().unwrap();
        let c = queue.remove(d).unwrap();
        // println!("Trying: [{d}]");
        if d.len() == 0 {
            // println!("Incrementing {acc} by {c}");
            acc += c;
            continue;
        }
        for pattern in patterns {
            if d.starts_with(pattern) {
                let key = &d[pattern.len()..];
                let mut val = c;
                if queue.contains_key(key) {
                    let v = queue.get(key).unwrap();
                    val += *v;
                }
                // println!("Inserting: {key} {val}");
                queue.insert(key, val);
            }
        }
    }

    // println!("Trying: {design} {acc}");

    acc
}

pub fn part_two(input: &str) -> Option<String> {
    let (patterns, designs) = parse_input(input).expect("Correct input format").1;

    let mut acc = 0;
    for design in designs {
        acc += count(&patterns, design);
    }

    Some(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("16".to_string()));
    }
}
