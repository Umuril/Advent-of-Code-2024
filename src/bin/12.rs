use advent_of_code::{Matrix, Point};

advent_of_code::solution!(12);

const ALL_DIRECTIONS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

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

        for direction in ALL_DIRECTIONS {
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
fn search_part_two(start: Point, matrix: &mut Matrix<u8>) -> (u32, u32) {
    let mut area = 0;
    let mut sides = 0i32;

    let mut queue = Vec::new();
    queue.push(start);

    let mut all_sides = Vec::new();

    while let Some(value) = queue.pop() {
        // println!("Value: {value:?}");
        // dbg!(&matrix);

        let old_value = matrix.update(&value, b'.').unwrap();
        if old_value == b'.' {
            continue;
        }

        area += 1;
        sides += 4;

        for direction in ALL_DIRECTIONS {
            all_sides.push((value, direction));
            let new_position = value + direction;
            let new_value = matrix.get(&new_position);
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

pub fn part_two(input: &str) -> Option<u64> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0;
    for p in matrix.as_points() {
        let cell = matrix.get(&p).unwrap();
        if *cell != b'.' {
            let (area, perimeter) = search_part_two(p, &mut matrix);
            // println!("{area} {perimeter}");
            acc += (area * perimeter) as u64;
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
        assert_eq!(result, None);
    }
}
