use advent_of_code::{template::commands::all, Matrix};

advent_of_code::solution!(12);

const ALL_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn search_part_one(start: (i32, i32), matrix: &mut Matrix<u8>) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0i32;

    let mut queue = Vec::new();
    queue.push(start);

    while !queue.is_empty() {
        let value = queue.pop().unwrap();
        // println!("Value: {value:?}");
        // dbg!(&matrix);

        let old_value = matrix.update(value.0, value.1, b'.').unwrap();
        if old_value == b'.' {
            continue;
        }

        area += 1;
        perimeter += 4;

        for direction in ALL_DIRECTIONS {
            let new_position = (value.0 + direction.0, value.1 + direction.1);
            let new_value = matrix.get(new_position.0, new_position.1);
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

    (area, perimeter as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0;
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let cell = matrix.get(r as i32, c as i32).unwrap();
            if *cell != b'.' {
                let (area, perimeter) = search_part_one((r as i32, c as i32), &mut matrix);
                // println!("{area} {perimeter}");
                acc += area * perimeter;
            }
        }
    }

    Some(acc)
}
fn search_part_two(start: (i32, i32), matrix: &mut Matrix<u8>) -> (u32, u32) {
    let mut area = 0;
    let mut sides = 0i32;

    let mut queue = Vec::new();
    queue.push(start);

    let mut all_sides = Vec::new();

    while !queue.is_empty() {
        let value = queue.pop().unwrap();
        // println!("Value: {value:?}");
        // dbg!(&matrix);

        let old_value = matrix.update(value.0, value.1, b'.').unwrap();
        if old_value == b'.' {
            continue;
        }

        area += 1;
        sides += 4;

        for direction in ALL_DIRECTIONS {
            all_sides.push((value, direction));
            let new_position = (value.0 + direction.0, value.1 + direction.1);
            let new_value = matrix.get(new_position.0, new_position.1);
            if let Some(v) = new_value {
                if *v == old_value {
                    sides -= 2;
                    queue.push(new_position);
                }
            } else {
                // Ok
            }
        }
    }

    (area, sides as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0;
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            let cell = matrix.get(r as i32, c as i32).unwrap();
            if *cell != b'.' {
                let (area, perimeter) = search_part_two((r as i32, c as i32), &mut matrix);
                // println!("{area} {perimeter}");
                acc += area * perimeter;
            }
        }
    }

    Some(acc);
    None
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
        assert_eq!(result, None);
    }
}
