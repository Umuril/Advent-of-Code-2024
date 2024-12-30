use itertools::Itertools;

use advent_of_code::{Matrix, Point, ALL_4_DIRECTIONS};

advent_of_code::solution!(12);

fn search_part_one(start: Point, matrix: &mut Matrix<u8>) -> (u64, u64) {
    let mut area = 0;
    let mut perimeter = 0i32;

    let mut queue = Vec::new();
    queue.push(start);

    while let Some(value) = queue.pop() {
        // println!("Value: {value:?}");
        // dbg!(&matrix);

        let old_value = matrix.update(&value, b'.').unwrap();
        if old_value == b'.' {
            continue;
        }

        area += 1;
        perimeter += 4;

        for direction in ALL_4_DIRECTIONS {
            let new_position = value + direction;
            let new_value = matrix.get(&new_position);
            if let Some(v) = new_value {
                if *v == old_value {
                    perimeter -= 2;
                    queue.push(new_position);
                }
            } else {
                // Ok
            }
        }
    }

    (area, perimeter as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0u64;
    for p in matrix.as_points() {
        let cell = matrix.get(&p).unwrap();
        if *cell != b'.' {
            let (area, perimeter) = search_part_one(p, &mut matrix);
            // println!("{p:?} {area} {perimeter}");
            acc += area * perimeter;
        }
    }

    Some(acc)
}
fn search_part_two(start: Point, matrix: &mut Matrix<u8>) -> (u64, u64) {
    let mut area = 0;
    let mut corners = 0;

    let mut queue = Vec::new();
    queue.push(start);

    while let Some(value) = queue.pop() {
        // println!("Value: {value:?}");
        // println!("Matrix:\n{matrix}");

        let old_value = matrix.update(&value, b'#').unwrap();

        let mut neightboors = Vec::new();
        for direction in ALL_4_DIRECTIONS {
            let new_position = value + direction;
            let new_value = matrix.get(&new_position);
            if let Some(v) = new_value {
                if *v == old_value || *v == b'#' {
                    neightboors.push(direction);
                }
            }
        }

        if old_value == b'#' || old_value == b'.' {
            continue;
        }

        corners += match neightboors.len() {
            0 => 4,
            1 => 2,
            2 => {
                if neightboors[0] == neightboors[1].opposite_direction() {
                    0 // Opposti
                } else {
                    let v = *matrix
                        .get(&(value + neightboors[0] + neightboors[1]))
                        .unwrap();
                    if v != old_value && v != b'#' {
                        2 // Angolo esterno e interno
                    } else {
                        1 // Solo angolo esterno
                    }
                }
            }
            3 | 4 => {
                let mut acc = 0;
                for combo in neightboors.iter().combinations(2) {
                    let v = *matrix.get(&((value + *combo[0]) + *combo[1])).unwrap();
                    if v != old_value && v != b'#' {
                        acc += 1;
                    }
                }
                acc
            }
            _ => unreachable!(),
        };

        // println!("Corners: {corners} - {value} - Old value: {} - {}", char::from_u32(old_value as u32).unwrap(), neightboors.len());

        area += 1;

        for direction in ALL_4_DIRECTIONS {
            let new_position = value + direction;
            let new_value = matrix.get(&new_position);
            if let Some(v) = new_value {
                if *v == old_value {
                    queue.push(new_position);
                }
            } else {
                // Ok
            }
        }
    }

    for p in matrix.as_points() {
        if *matrix.get(&p).unwrap() == b'#' {
            matrix.update(&p, b'.').unwrap();
        }
    }

    (area, corners)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0;
    for p in matrix.as_points() {
        let cell = *matrix.get(&p).unwrap();
        if cell != b'.' {
            let (area, perimeter) = search_part_two(p, &mut matrix);
            // println!("{} {area} {perimeter}", char::from_u32(cell as u32).unwrap());
            acc += area * perimeter;
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
