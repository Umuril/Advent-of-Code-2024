use advent_of_code::{Matrix, Point, ALL_4_DIRECTIONS, LEFT};
use itertools::Itertools;
use pathfinding::prelude::astar;

advent_of_code::solution!(20);

fn find_successors(matrix: &Matrix<u8>, point: &Point) -> Vec<(Point, usize)> {
    let mut next = Vec::new();
    for direction in ALL_4_DIRECTIONS {
        let new_point = *point + direction;
        if let Some(chr) = matrix.get(&new_point) {
            if *chr != b'#' {
                next.push((new_point, 1));
            }
        }
    }

    next
}

pub fn part_one(input: &str) -> Option<String> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let start = matrix.find(&b'S').unwrap();
    let end = matrix.find(&b'E').unwrap();

    // println!("{matrix}\n{start}\n{end}");

    let perfect = astar(
        &start,
        |p| find_successors(&matrix, p),
        |p| p.distance(end) as usize,
        |p| *p == end,
    )
    .expect("ERROR: At least one solution to the maze!")
    .1;

    let mut acc = 0;

    for p in matrix.as_points() {
        if *matrix.get(&p).unwrap() == b'#' {
            let mut new_matrix = matrix.clone();
            new_matrix.update(&p, b'.');
            let result = astar(
                &start,
                |p| find_successors(&new_matrix, p),
                |p| p.distance(end) as usize,
                |p| *p == end,
            )
            .expect("ERROR: At least one solution to the maze!")
            .1;

            let saved = perfect - result;
            let threshold = if cfg!(test) { 1 } else { 100 };
            if saved >= threshold {
                acc += 1;
            }
        }
    }

    Some(acc.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let start = matrix.find(&b'S').unwrap();
    let end = matrix.find(&b'E').unwrap();
    let mut current_direction = LEFT;
    let mut current_position = end;
    let mut cost = 0;

    let mut all_points = Vec::new();
    all_points.push((end, 0));

    loop {
        for direction in ALL_4_DIRECTIONS {
            if direction == current_direction.opposite_direction() {
                continue;
            }
            let new_position = current_position + direction;
            let new_chr = matrix.get(&new_position).unwrap();
            if *new_chr != b'#' {
                current_position = new_position;
                current_direction = direction;
                break;
            }
        }

        cost += 1;
        all_points.push((current_position, cost));

        if current_position == start {
            break;
        }
    }

    let mut acc = 0;
    for pair in all_points.into_iter().permutations(2) {
        let (a, b) = (pair.first().unwrap(), pair.last().unwrap());

        let threshold = if cfg!(test) { 50 } else { 100 };
        if a.0.distance(b.0) <= 20 && a.1 - b.1 - a.0.distance(b.0) >= threshold {
            acc += 1;
        }
    }

    Some(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("44".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("285".to_string()));
    }
}
