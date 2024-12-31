use advent_of_code::{Matrix, Point, ALL_4_DIRECTIONS};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    multi::separated_list1,
    sequence::separated_pair,
};
use pathfinding::prelude::astar;

advent_of_code::solution!(18);

fn find_successors(matrix: &Matrix<u8>, point: &Point) -> Vec<(Point, usize)> {
    let mut next = Vec::new();
    for direction in ALL_4_DIRECTIONS {
        let new_point = *point + direction;
        if let Some(chr) = matrix.get(&new_point) {
            if *chr == b'.' {
                next.push((new_point, 1));
            }
        }
    }

    next
}

pub fn part_one(input: &str) -> Option<String> {
    let new_line = line_ending::<&str, ()>;
    let data = separated_list1(new_line, separated_pair(u32, tag(","), u32))(input)
        .expect("Correct input format")
        .1;

    let size = if cfg!(test) { 7 } else { 71 };
    let num_rocks = if cfg!(test) { 12 } else { 1024 };

    let mut matrix = Matrix::empty(size, size, b'.');

    let start = Point(0, 0);
    let end = Point(size as isize - 1, size as isize - 1);

    for (y, x) in &data[..num_rocks] {
        matrix.update(&Point(*x as isize, *y as isize), b'#');
    }

    let result = astar(
        &start,
        |p| find_successors(&matrix, p),
        |p| ((p.0 - end.0).abs() + (p.1 - end.1).abs()) as usize,
        |p| *p == end,
    )
    .unwrap()
    .1;

    Some(result.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let new_line = line_ending::<&str, ()>;
    let data = separated_list1(new_line, separated_pair(u32, tag(","), u32))(input)
        .expect("Correct input format")
        .1;

    let size = if cfg!(test) { 7 } else { 71 };
    let num_rocks = if cfg!(test) { 12 } else { 1024 };

    let mut matrix = Matrix::empty(size, size, b'.');

    let start = Point(0, 0);
    let end = Point(size as isize - 1, size as isize - 1);

    for (y, x) in &data[..num_rocks] {
        matrix.update(&Point(*x as isize, *y as isize), b'#');
    }

    let mut rock = 0;
    loop {
        let (y, x) = data[rock];
        matrix.update(&Point(x as isize, y as isize), b'#');

        let result = astar(
            &start,
            |p| find_successors(&matrix, p),
            |p| ((p.0 - end.0).abs() + (p.1 - end.1).abs()) as usize,
            |p| *p == end,
        );

        if result.is_none() {
            break;
        }

        rock += 1;
    }

    let (y, x) = data[rock];

    Some(format!("{y},{x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("22".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
