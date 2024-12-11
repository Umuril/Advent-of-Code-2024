use std::collections::HashSet;

use advent_of_code::Matrix;

advent_of_code::solution!(6);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up(i32, i32),
    Down(i32, i32),
    Left(i32, i32),
    Right(i32, i32),
}

impl Direction {
    fn get_coordinates(self) -> (i32, i32) {
        match self {
            Direction::Up(dr, dc) => (dr, dc),
            Direction::Down(dr, dc) => (dr, dc),
            Direction::Left(dr, dc) => (dr, dc),
            Direction::Right(dr, dc) => (dr, dc),
        }
    }
}

static UP: Direction = Direction::Up(-1, 0);
static DOWN: Direction = Direction::Down(1, 0);
static LEFT: Direction = Direction::Left(0, -1);
static RIGHT: Direction = Direction::Right(0, 1);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    fn forward(&mut self, matrix: &mut Matrix<u8>) -> Option<i32> {
        let (dr, dc) = self.direction.get_coordinates();
        let new_position = (self.position.0 + dr, self.position.1 + dc);

        matrix.get(self.position.0, self.position.1)?;

        let new_chr = matrix.get(new_position.0, new_position.1).unwrap_or(&b'@');

        if *new_chr == b'#' {
            self.rotate();
            return Some(0);
        }
        let current_pos = matrix
            .get(self.position.0, self.position.1)
            .expect("Checked");

        if *current_pos == b'X' {
            self.position = new_position;
            return Some(0);
        }

        matrix.update(self.position.0, self.position.1, b'X');
        self.position = new_position;

        Some(1)
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up(_, _) => RIGHT,
            Direction::Down(_, _) => LEFT,
            Direction::Left(_, _) => UP,
            Direction::Right(_, _) => DOWN,
        }
    }
}
fn search_guard(matrix: &Matrix<u8>) -> Option<Guard> {
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let direction = match matrix.get(r as i32, c as i32) {
                Some(b'^') => UP,
                Some(b'v') => DOWN,
                Some(b'>') => LEFT,
                Some(b'<') => RIGHT,
                _ => continue,
            };
            return Some(Guard {
                position: (r as i32, c as i32),
                direction,
            });
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut guard = search_guard(&matrix).expect("At least one guard");

    let mut acc = 0u32;
    loop {
        let steps = guard.forward(&mut matrix);
        match steps {
            Some(step) => acc += step as u32,
            None => break,
        }
    }

    Some(acc)
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

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let guard = search_guard(&matrix).expect("At least one guard");

    let mut possible_positions = HashSet::with_capacity(matrix.rows * matrix.cols);

    let mut guarded = guard.clone();
    let mut cloned = matrix.clone();
    loop {
        possible_positions.insert(guarded.position);
        if guarded.forward(&mut cloned).is_none() {
            break;
        }
    }

    let mut acc = 0u32;
    for pos in possible_positions {
        let mut test = matrix.clone();
        test.update(pos.0, pos.1, b'#');
        if search_loop(guard.clone(), test).is_some() {
            acc += 1;
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
