use core::str;

use advent_of_code::{Matrix, Point};
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(14);

const MAX_X: u32 = 101;
const MAX_Y: u32 = 103;
const MID_X: u32 = MAX_X / 2;
const MID_Y: u32 = MAX_Y / 2;

pub fn part_one(input: &str) -> Option<u64> {
    let new_line = line_ending::<&str, ()>;
    let result = separated_list1(
        new_line,
        separated_pair(
            preceded(tag("p="), separated_pair(i32, tag(","), i32)),
            space1,
            preceded(tag("v="), separated_pair(i32, tag(","), i32)),
        ),
    )(input);
    let data = result.expect("Correct input format").1;

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

    for ((px, py), (vx, vy)) in data {
        let new_px = (px + vx * 100).rem_euclid(MAX_X as i32) as u32;
        let new_py = (py + vy * 100).rem_euclid(MAX_Y as i32) as u32;

        match (new_px, new_py) {
            (MID_X..=MID_X, _) => {}
            (_, MID_Y..=MID_Y) => {}
            (0..MID_X, 0..MID_Y) => {
                a += 1;
            }
            (0..MID_X, MID_Y..MAX_Y) => {
                b += 1;
            }
            (MID_X..MAX_X, 0..MID_Y) => {
                c += 1;
            }
            (MID_X..MAX_X, MID_Y..MAX_Y) => {
                d += 1;
            }
            _ => {
                unreachable!()
            }
        }
    }

    Some(a * b * c * d)
}

pub fn part_two(input: &str) -> Option<u64> {
    let new_line = line_ending::<&str, ()>;
    let result = separated_list1(
        new_line,
        separated_pair(
            preceded(tag("p="), separated_pair(i32, tag(","), i32)),
            space1,
            preceded(tag("v="), separated_pair(i32, tag(","), i32)),
        ),
    )(input);
    let parsed_data = result.expect("Correct input format").1;
    let data: Vec<(Point, Point)> = parsed_data
        .iter()
        .map(|((px, py), (vx, vy))| {
            (
                Point(*px as isize, *py as isize),
                Point(*vx as isize, *vy as isize),
            )
        })
        .collect();

    for sec in 0..=101 * 103 {
        let mut matrix = Matrix::from(
            MAX_X as usize,
            MAX_Y as usize,
            [b'.'; MAX_X as usize * MAX_Y as usize].to_vec(),
        );
        for (p, v) in &data {
            let new_px = (p.0 + v.0 * sec).rem_euclid(MAX_X as isize);
            let new_py = (p.1 + v.1 * sec).rem_euclid(MAX_Y as isize);

            matrix.update(&Point(new_px, new_py), b'#');
        }

        if let Ok(s) = str::from_utf8(matrix.data.as_slice()) {
            if s.contains("###########") {
                return Some(sec as u64);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6446));
    }
}
