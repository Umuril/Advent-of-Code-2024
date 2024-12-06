use std::{
    collections::HashSet,
    fmt::{Debug, Write},
};

use nom::InputIter;

advent_of_code::solution!(6);

#[derive(Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl Matrix {
    fn from(rows: usize, cols: usize, str: &str) -> Matrix {
        assert_eq!(str.len(), rows * cols);
        Matrix {
            cols,
            rows,
            data: str.iter_elements().collect(),
        }
    }

    fn get(&self, row: i32, col: i32) -> Option<char> {
        if (0..self.rows as i32).contains(&row) && (0..self.cols as i32).contains(&col) {
            let pos = row as usize * self.cols + col as usize;
            let chr = *self.data.get(pos).expect("Checked");
            return Some(chr);
        }
        None
    }

    fn update(&mut self, row: i32, col: i32, chr: char) -> Option<char> {
        if (0..self.rows as i32).contains(&row) && (0..self.cols as i32).contains(&col) {
            let pos = row as usize * self.cols + col as usize;
            let old = *self.data.get(pos).expect("Checked");
            *self.data.get_mut(pos).expect("Checked") = chr;
            return Some(old);
        }
        None
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            f.write_char('\n')?;
            for c in 0..self.cols {
                f.write_char(self.get(r as i32, c as i32).expect("Checked"))?;
            }
        }
        Ok(())
    }
}

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
    fn forward(&mut self, matrix: &mut Matrix) -> Option<i32> {
        let (dr, dc) = self.direction.get_coordinates();
        let new_position = (self.position.0 + dr, self.position.1 + dc);

        matrix.get(self.position.0, self.position.1)?;

        let new_chr = matrix.get(new_position.0, new_position.1).unwrap_or('@');

        if new_chr == '#' {
            self.rotate();
            return Some(0);
        }
        let current_pos = matrix
            .get(self.position.0, self.position.1)
            .expect("Checked");

        if current_pos == 'X' {
            self.position = new_position;
            return Some(0);
        }

        matrix.update(self.position.0, self.position.1, 'X');
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
fn search_guard(matrix: &Matrix) -> Option<Guard> {
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let direction = match matrix.get(r as i32, c as i32) {
                Some('^') => UP,
                Some('v') => DOWN,
                Some('>') => LEFT,
                Some('<') => RIGHT,
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

    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").as_str());

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

fn search_loop(guard: Guard, mut matrix: Matrix) -> Option<bool> {
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

    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").as_str());

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
        test.update(pos.0, pos.1, '#');
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
