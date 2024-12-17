use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::Matrix;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut letters: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let chr = matrix.get(r as i32, c as i32).expect("Checked");
            if *chr != b'.' {
                letters.entry(*chr).or_default().push((r, c));
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

    Some(result.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut letters: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let chr = matrix.get(r as i32, c as i32).expect("Checked");
            if *chr != b'.' {
                letters.entry(*chr).or_default().push((r, c));
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

    Some(result.len() as u64)
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
