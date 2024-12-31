use std::collections::{HashMap, HashSet};

use advent_of_code::{Direction, Matrix, Point, ALL_4_DIRECTIONS, RIGHT};

advent_of_code::solution!(16);

fn dfs_part_one(matrix: &mut Matrix<Cell>, start: Point) -> Option<String> {
    let mut queue: Vec<(Direction, Point, u64)> = Vec::new();
    queue.push((RIGHT, start, 0));

    while let Some((direction, p, current_points)) = queue.pop() {
        for dir in ALL_4_DIRECTIONS {
            let next_point = p + dir;
            let next_cell = matrix.get_mut(&next_point).unwrap();

            if next_cell.chr == b'#' {
                continue;
            }

            let new_points = current_points
                + if dir == direction {
                    1
                } else if dir == direction.opposite_direction() {
                    2001
                } else {
                    1001
                };

            if let Some(old_points) = next_cell.next.get(&dir) {
                if new_points < *old_points {
                    next_cell.next.insert(dir, new_points);
                    queue.push((dir, next_point, new_points));
                }
            } else {
                next_cell.next.insert(dir, new_points);
                queue.push((dir, next_point, new_points));
            }
        }
    }

    // print_matrix(&matrix);

    let end = matrix.find(&Cell::new(b'E')).unwrap();

    let end_cell = matrix.get(&end).unwrap();

    get_optimal(end_cell).map(|x| x.to_string())
}

#[derive(Clone)]
struct Cell {
    chr: u8,
    next: HashMap<Direction, u64>,
}
impl Cell {
    fn new(chr: u8) -> Self {
        Self {
            chr,
            next: HashMap::new(),
        }
    }
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.chr == other.chr
    }
}

// fn print_matrix(matrix: &Matrix<Cell>) {
//     for x in 0..matrix.rows {
//         if x > 0 {
//             println!("");
//         }
//         for y in 0..matrix.cols {
//             let value = matrix.get(&Point(x, y)).expect("Checked");
//             print!("{}", char::from_u32(value.chr as u32).unwrap());
//             // for n in &value.next {
//             //     print!("{}{}/", n.0, n.1);
//             // }
//             print!("");
//         }
//     }
// }

pub fn part_one(input: &str) -> Option<String> {
    let nrows = input
        .trim()
        .as_bytes()
        .iter()
        .filter(|&&c| c == b'\n')
        .count()
        + 1;
    let ncols = input.trim().find('\n').unwrap();
    let matrix = Matrix::from(
        nrows,
        ncols,
        input
            .trim()
            .replace('\n', "")
            .as_bytes()
            .iter()
            .map(|chr| Cell::new(*chr))
            .collect(),
    );

    let start = matrix.find(&Cell::new(b'S')).unwrap();

    // println!("{matrix}");
    // println!("{start} {end}");

    dfs_part_one(&mut matrix.clone(), start)
}

fn get_optimal(cell: &Cell) -> Option<u64> {
    cell.next.values().min().copied()
}

fn dfs_part_two(matrix: &mut Matrix<Cell>, start: Point) -> Option<String> {
    let mut queue: Vec<(Direction, Point, u64)> = Vec::new();
    queue.push((RIGHT, start, 0));

    while let Some((direction, p, current_points)) = queue.pop() {
        for dir in ALL_4_DIRECTIONS {
            let next_point = p + dir;
            let next_cell = matrix.get_mut(&next_point).unwrap();

            if next_cell.chr == b'#' {
                continue;
            }

            let new_points = current_points
                + if dir == direction {
                    1
                } else if dir == direction.opposite_direction() {
                    2001
                } else {
                    1001
                };

            if let Some(old_points) = next_cell.next.get(&dir) {
                if new_points < *old_points {
                    next_cell.next.insert(dir, new_points);
                    queue.push((dir, next_point, new_points));
                }
            } else {
                next_cell.next.insert(dir, new_points);
                queue.push((dir, next_point, new_points));
            }
        }
    }

    // print_matrix(&matrix);

    let end = matrix.find(&Cell::new(b'E')).unwrap();

    let end_cell = matrix.get(&end).unwrap();
    let optimal_value = get_optimal(end_cell).unwrap();

    for n in &end_cell.next {
        if *n.1 == optimal_value {
            queue.push((*n.0, end, *n.1));
        }
    }

    let mut paths = HashSet::new();

    while let Some((direction, p, current_points)) = queue.pop() {
        // for dir in ALL_4_DIRECTIONS {
        let old_point = p + direction.opposite_direction();
        let old_cell = matrix.get(&old_point).unwrap();

        for (dir, points) in &old_cell.next {
            let expected_points = current_points
                - if *dir == direction {
                    1
                } else if *dir == direction.opposite_direction() {
                    2001
                } else {
                    1001
                };

            if expected_points == *points {
                paths.insert(old_point);
                queue.push((*dir, old_point, expected_points));
            }
        }
    }

    for path in &paths {
        matrix.get_mut(path).unwrap().chr = b'O';
    }

    // print_matrix(&matrix);

    Some((2 + paths.len() as u64).to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let nrows = input
        .trim()
        .as_bytes()
        .iter()
        .filter(|&&c| c == b'\n')
        .count()
        + 1;
    let ncols = input.trim().find('\n').unwrap();
    let matrix = Matrix::from(
        nrows,
        ncols,
        input
            .trim()
            .replace('\n', "")
            .as_bytes()
            .iter()
            .map(|chr| Cell::new(*chr))
            .collect(),
    );

    let start = matrix.find(&Cell::new(b'S')).unwrap();

    // println!("{matrix}");
    // println!("{start} {end}");

    dfs_part_two(&mut matrix.clone(), start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("11048".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("64".to_string()));
    }
}
