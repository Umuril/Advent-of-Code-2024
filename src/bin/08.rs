use nom::InputIter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Write;

advent_of_code::solution!(8);

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

    fn _update(&mut self, row: i32, col: i32, chr: char) -> Option<char> {
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
pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").as_str());

    let mut letters: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let chr = matrix.get(r as i32, c as i32).expect("Checked");
            if chr != '.' {
                letters.entry(chr).or_default().push((r, c));
            }
        }
    }

    let mut result = HashSet::new();
    for (_chr, letter) in letters.into_iter() {
        for p1 in &letter {
            for p2 in &letter {
                let new_point = (
                    p1.0 as i32 - (p2.0 as i32 - p1.0 as i32),
                    p1.1 as i32 - (p2.1 as i32 - p1.1 as i32),
                );
                if p1 != p2 && matrix.get(new_point.0, new_point.1).is_some() {
                    // println!("{:?} {:?} {:?} {:?}", p1, p2, chr, new_point);
                    result.insert(new_point);
                }
            }
        }
    }

    Some(result.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").as_str());

    let mut letters: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let chr = matrix.get(r as i32, c as i32).expect("Checked");
            if chr != '.' {
                letters.entry(chr).or_default().push((r, c));
            }
        }
    }

    let mut result = HashSet::new();
    for (_chr, letter) in letters.into_iter() {
        for p1 in &letter {
            for p2 in &letter {
                if p1 == p2 {
                    result.insert((p1.0 as i32, p1.1 as i32));
                    continue;
                }
                let diff = (p2.0 as i32 - p1.0 as i32, p2.1 as i32 - p1.1 as i32);
                let mut new_point = (p1.0 as i32 - diff.0, p1.1 as i32 - diff.1);
                while matrix.get(new_point.0, new_point.1).is_some() {
                    result.insert(new_point);
                    new_point = (new_point.0 - diff.0, new_point.1 - diff.1);
                }
            }
        }
    }

    Some(result.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
