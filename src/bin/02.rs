use nom::{
    character::{
        complete::u32,
        complete::{line_ending, space1},
    },
    multi::separated_list1,
};

advent_of_code::solution!(2);

fn check_row(row: &[u32]) -> bool {
    let order = row.first().cmp(&row.get(1));
    row.windows(2)
        .map(|pair| (pair[0], pair[1]))
        .all(|(x, y)| x.cmp(&y) == order && x.abs_diff(y) <= 3)
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let new_line = line_ending::<&str, ()>;
    let result = separated_list1(new_line, separated_list1(space1, u32))(input);
    result.expect("Correct input format").1
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input);

    let total = data.iter().filter(|row| check_row(row)).count();

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(input);

    let total = data
        .iter()
        .filter(|row| {
            if check_row(row) {
                return true;
            }

            for n in 0..row.len() {
                let new_row = row
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != n)
                    .map(|(_, v)| *v)
                    .collect::<Vec<u32>>();

                if check_row(&new_row) {
                    return true;
                }
            }

            false
        })
        .count();

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
