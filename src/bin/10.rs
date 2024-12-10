use std::collections::HashSet;

use advent_of_code::Matrix;

advent_of_code::solution!(10);

const ALL_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn dfs_part_one(position: (i32, i32), matrix: &Matrix<u8>) -> HashSet<(i32, i32)> {
    let mut acc = HashSet::new();
    let current_value = matrix.get(position.0, position.1).expect("Checked");
    // println!("{:?} {}", position, *current_value - b'0');
    if *current_value == b'9' {
        acc.insert(position);
        return acc;
    }
    for direction in ALL_DIRECTIONS {
        let new_position = (position.0 + direction.0, position.1 + direction.1);
        if let Some(p) = matrix.get(new_position.0, new_position.1) {
            if *p == *current_value + 1 {
                let new_ends = dfs_part_one(new_position, matrix);
                for end in new_ends {
                    acc.insert(end);
                }
            }
        }
    }

    acc
}
pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut starts = Vec::new();
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let cell = matrix.get(r as i32, c as i32).expect("Checked");
            if *cell == b'0' {
                starts.push((r as i32, c as i32));
            }
        }
    }

    let mut acc = 0;
    for start in starts {
        acc += dfs_part_one(start, &matrix).len();
    }

    Some(acc as u32)
}

fn dfs_part_two(position: (i32, i32), matrix: &Matrix<u8>) -> u32 {
    let mut acc = 0;
    let current_value = matrix.get(position.0, position.1).expect("Checked");
    // println!("{:?} {}", position, *current_value - b'0');
    if *current_value == b'9' {
        return 1;
    }
    for direction in ALL_DIRECTIONS {
        let new_position = (position.0 + direction.0, position.1 + direction.1);
        if let Some(p) = matrix.get(new_position.0, new_position.1) {
            if *p == *current_value + 1 {
                acc += dfs_part_two(new_position, matrix);
            }
        }
    }

    acc
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut starts = Vec::new();
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let cell = matrix.get(r as i32, c as i32).expect("Checked");
            if *cell == b'0' {
                starts.push((r as i32, c as i32));
            }
        }
    }

    let mut acc = 0;
    for start in starts {
        acc += dfs_part_two(start, &matrix);
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
