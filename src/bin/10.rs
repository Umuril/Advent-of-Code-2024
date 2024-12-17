use std::collections::HashSet;

use advent_of_code::{Matrix, Point};

advent_of_code::solution!(10);

const ALL_DIRECTIONS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];
fn dfs_part_one(position: Point, matrix: &Matrix<u8>) -> HashSet<Point> {
    let mut acc = HashSet::new();
    let current_value = matrix.get(&position).expect("Checked");
    // println!("{:?} {}", position, *current_value - b'0');
    if *current_value == b'9' {
        acc.insert(position);
        return acc;
    }
    for direction in ALL_DIRECTIONS {
        let new_position = position + direction;
        if let Some(p) = matrix.get(&new_position) {
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
pub fn part_one(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut starts = Vec::new();
    for p in matrix.as_points() {
        let cell = matrix.get(&p).expect("Checked");
        if *cell == b'0' {
            starts.push(p);
        }
    }

    let mut acc = 0;
    for start in starts {
        acc += dfs_part_one(start, &matrix).len() as u64;
    }

    Some(acc)
}

fn dfs_part_two(position: Point, matrix: &Matrix<u8>) -> u64 {
    let mut acc = 0;
    let current_value = matrix.get(&position).expect("Checked");
    // println!("{:?} {}", position, *current_value - b'0');
    if *current_value == b'9' {
        return 1;
    }
    for direction in ALL_DIRECTIONS {
        let new_position = position + direction;
        if let Some(p) = matrix.get(&new_position) {
            if *p == *current_value + 1 {
                acc += dfs_part_two(new_position, matrix);
            }
        }
    }

    acc
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut starts = Vec::new();
    for p in matrix.as_points() {
        let cell = matrix.get(&p).expect("Checked");
        if *cell == b'0' {
            starts.push(p);
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
