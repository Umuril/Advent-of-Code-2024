use nom::{
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(4);

const ALL_DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn search_from(start: (i32, i32), rows: &Vec<&str>) -> u32 {
    let mut acc = 0;
    for direction in ALL_DIRECTIONS {
        let mut pointer = start;
        let mut found = true;
        for expected in "MAS".chars() {
            pointer = (pointer.0 + direction.0, pointer.1 + direction.1);
            if pointer.0 < 0 || pointer.0 as usize >= rows.len() {
                found = false;
                break;
            }

            if let Some(&row) = rows.get(pointer.0 as usize) {
                if pointer.1 < 0 || pointer.1 as usize >= row.len() {
                    found = false;
                    break;
                }
                if let Some(chr) = row.chars().nth(pointer.1 as usize) {
                    if chr != expected {
                        found = false;
                        break;
                    }
                }
            }
        }
        if found {
            acc += 1;
        }
    }
    acc
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: IResult<&str, Vec<&str>> = separated_list1(line_ending, alpha1)(input);
    let rows: Vec<&str> = result.expect("Correct input format").1;

    let mut starts = Vec::new();

    for (r, row) in rows.iter().enumerate() {
        for (c, chr) in row.chars().enumerate() {
            if chr == 'X' {
                starts.push((r as i32, c as i32));
            }
        }
    }

    let mut acc = 0;
    for start in starts {
        acc += search_from(start, &rows);
    }

    Some(acc)
}

fn search_cross(start: (i32, i32), rows: &Vec<&str>) -> u32 {
    let a = rows
        .get(start.0 as usize - 1)
        .unwrap()
        .chars()
        .nth(start.1 as usize - 1)
        .unwrap();
    let b = rows
        .get(start.0 as usize - 1)
        .unwrap()
        .chars()
        .nth(start.1 as usize + 1)
        .unwrap();
    let c = rows
        .get(start.0 as usize + 1)
        .unwrap()
        .chars()
        .nth(start.1 as usize - 1)
        .unwrap();
    let d = rows
        .get(start.0 as usize + 1)
        .unwrap()
        .chars()
        .nth(start.1 as usize + 1)
        .unwrap();
    match (a, b, c, d) {
        ('M', 'M', 'S', 'S')
        | ('S', 'M', 'S', 'M')
        | ('S', 'S', 'M', 'M')
        | ('M', 'S', 'M', 'S') => 1,
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: IResult<&str, Vec<&str>> = separated_list1(line_ending, alpha1)(input);
    let rows: Vec<&str> = result.expect("Correct input format").1;

    let mut starts = Vec::new();

    for (r, row) in rows.iter().enumerate() {
        for (c, chr) in row.chars().enumerate() {
            if r != 0 && r != rows.len() - 1 && c != 0 && c != row.len() - 1 && chr == 'A' {
                starts.push((r as i32, c as i32));
            }
        }
    }

    let mut acc = 0;
    for start in starts {
        acc += search_cross(start, &rows);
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
