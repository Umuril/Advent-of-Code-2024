use core::str;

use advent_of_code::{Matrix, DOWN, LEFT, RIGHT, UP};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let (part_a, part_b) = input
        .trim()
        .split_once("\n\n")
        .expect("Correct input format");
    let nrows = part_a.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
    let ncols = part_a.find('\n').unwrap();
    let part_a = &part_a.replace('\n', "");
    let mut matrix = Matrix::from(nrows, ncols, part_a.as_bytes().to_vec());
    // println!("{}", matrix);

    let part_b = &part_b.replace('\n', "");
    let moves: Vec<u8> = part_b.as_bytes().to_vec();
    // println!("{:?}", moves);

    let mut guard_pos = matrix.find(&b'@').expect("Expect one guard");

    for mov in moves {
        let direction = match mov {
            b'^' => UP,
            b'v' => DOWN,
            b'<' => LEFT,
            b'>' => RIGHT,
            _ => unreachable!(),
        };

        let mut next_pos = guard_pos + direction.as_point();

        // println!("GUARD: {} - NEXT: {} - DIRECTION: {}", guard_pos, next_pos, direction);

        while let Some(next_byte) = matrix.get(&next_pos) {
            // println!("NEXT BYTE: {}", str::from_utf8(&[*next_byte]).unwrap());
            if *next_byte == b'#' {
                break;
            }
            if *next_byte == b'O' {
                next_pos += direction.as_point();
                continue;
            }
            if *next_byte == b'.' {
                while next_pos != guard_pos {
                    let last_point = next_pos + direction.opposite_point();
                    let old_chr = matrix.get(&last_point).unwrap();
                    // println!("LAST: {} - OLD CHR: {}", last_point, old_chr);
                    matrix.update(&next_pos, *old_chr);
                    matrix.update(&last_point, b'.');

                    next_pos += direction.opposite_point();
                }
                guard_pos += direction.as_point();

                break;
            }
        }
    }

    let mut acc = 0;
    for p in matrix.as_points() {
        if let Some(chr) = matrix.get(&p) {
            if *chr == b'O' {
                acc += (p.0 * 100 + p.1) as u64;
            }
        }
    }

    Some(acc)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
