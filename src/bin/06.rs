use std::collections::HashSet;

use advent_of_code::{Matrix, Point};

advent_of_code::solution!(6);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

impl Direction {
    fn get_coordinates(self) -> Point {
        match self {
            Direction::Up(p) => p,
            Direction::Down(p) => p,
            Direction::Left(p) => p,
            Direction::Right(p) => p,
        }
    }
}

static UP: Direction = Direction::Up(Point(-1, 0));
static DOWN: Direction = Direction::Down(Point(1, 0));
static LEFT: Direction = Direction::Left(Point(0, -1));
static RIGHT: Direction = Direction::Right(Point(0, 1));

#[derive(Clone, PartialEq, Hash)]
struct Guard {
    position: Point,
    direction: Direction,
}

impl Guard {
    fn forward(&mut self, matrix: &mut Matrix<u8>) -> Option<i32> {
        let delta = self.direction.get_coordinates();
        let new_position = self.position + delta;

        matrix.get(&self.position)?;

        let new_chr = matrix.get(&new_position).unwrap_or(&b'@');

        if *new_chr == b'#' {
            self.rotate();
            return Some(0);
        }
        let current_pos = matrix.get(&self.position).expect("Checked");

        if *current_pos == b'X' {
            self.position = new_position;
            return Some(0);
        }

        matrix.update(&self.position, b'X');
        self.position = new_position;

        Some(1)
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up(_) => RIGHT,
            Direction::Down(_) => LEFT,
            Direction::Left(_) => UP,
            Direction::Right(_) => DOWN,
        }
    }
}
fn search_guard(matrix: &Matrix<u8>) -> Option<Guard> {
    for p in matrix.as_points() {
        let direction = match matrix.get(&p) {
            Some(b'^') => UP,
            Some(b'v') => DOWN,
            Some(b'>') => LEFT,
            Some(b'<') => RIGHT,
            _ => continue,
        };
        return Some(Guard {
            position: p,
            direction,
        });
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut guard = search_guard(&matrix).expect("At least one guard");

    let mut acc = 0;
    loop {
        let steps = guard.forward(&mut matrix);
        match steps {
            Some(step) => acc += step as u64,
            None => break,
        }
    }

    Some(acc.to_string())
}

fn search_loop(guard: Guard, mut matrix: Matrix<u8>) -> Option<bool> {
    let mut slow = guard.clone();
    let mut fast = guard.clone();
    loop {
        fast.forward(&mut matrix)?;
        fast.forward(&mut matrix)?;
        slow.forward(&mut matrix)?;

        if slow == fast {
            return Some(true);
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let guard = search_guard(&matrix).expect("At least one guard");

    let mut possible_positions = HashSet::with_capacity((matrix.rows * matrix.cols) as usize);

    let mut guarded = guard.clone();
    let mut cloned = matrix.clone();
    loop {
        possible_positions.insert(guarded.position);
        if guarded.forward(&mut cloned).is_none() {
            break;
        }
    }

    let mut acc = 0;
    for pos in possible_positions {
        let mut test = matrix.clone();
        test.update(&pos, b'#');
        if search_loop(guard.clone(), test).is_some() {
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
        assert_eq!(result, Some("41".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6".to_string()));
    }
}
