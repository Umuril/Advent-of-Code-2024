use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::Matrix;
use advent_of_code::Point;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut letters: HashMap<u8, Vec<Point>> = HashMap::new();
    for p in matrix.as_points() {
        let chr = matrix.get(&p).expect("Checked");
        if *chr != b'.' {
            letters.entry(*chr).or_default().push(p);
        }
    }

    let mut result = HashSet::new();
    for (_chr, letter) in letters.into_iter() {
        for p1 in &letter {
            for p2 in &letter {
                let new_point = Point(
                    (p1.0 as i32 - (p2.0 as i32 - p1.0 as i32)) as isize,
                    (p1.1 as i32 - (p2.1 as i32 - p1.1 as i32)) as isize,
                );
                if p1 != p2 && matrix.get(&new_point).is_some() {
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

    let mut letters: HashMap<u8, Vec<Point>> = HashMap::new();
    for p in matrix.as_points() {
        let chr = matrix.get(&p).expect("Checked");
        if *chr != b'.' {
            letters.entry(*chr).or_default().push(p);
        }
    }

    let mut result = HashSet::new();
    for (_chr, letter) in letters.into_iter() {
        for p1 in &letter {
            for p2 in &letter {
                if p1 == p2 {
                    result.insert(Point(p1.0, p1.1));
                    continue;
                }
                let diff = p2 - p1;
                let mut new_point = p1 - diff;
                while matrix.get(&new_point).is_some() {
                    result.insert(new_point);
                    new_point -= diff;
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
