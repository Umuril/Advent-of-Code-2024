use nom::InputIter;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let lengths: Vec<u32> = input
        .trim()
        .iter_elements()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let mut solution = Vec::<i32>::with_capacity(100_000);

    let mut current_len = 0usize;
    let mut current_idx = 0u32;

    for (i, l) in lengths.iter().enumerate() {
        current_len += *l as usize;

        if i % 2 == 0 {
            solution.resize_with(current_len, || current_idx as i32);
            current_idx += 1;
        } else {
            solution.resize_with(current_len, || -1);
        }
    }

    let mut i = 0;
    let mut j = solution.len() - 1;
    loop {
        if i > j {
            break;
        }
        if solution[i] == -1 {
            while solution[j] == -1 {
                j -= 1;
            }
            solution.swap(i, j);
            j -= 1;
        }
        i += 1;
    }

    let result = solution
        .iter()
        .filter(|x| **x >= 0)
        .map(|x| *x as usize)
        .enumerate()
        .fold(0, |x, y| x + y.0 * y.1);

    Some(result as u64)
}

#[derive(Debug, Clone)]
enum Type {
    Full(u32, u32), // Len + IDX
    Empty(u32),     // Len
}

pub fn part_two(input: &str) -> Option<u64> {
    let lengths: Vec<u32> = input
        .trim()
        .iter_elements()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut current_idx = 0i32;
    let mut values = Vec::with_capacity(lengths.len());
    for (i, l) in lengths.iter().enumerate() {
        if i % 2 == 0 {
            values.push(Type::Full(*l, current_idx as u32));
            current_idx += 1;
        } else {
            values.push(Type::Empty(*l));
        }
    }

    println!("{:#?}", &values);
    loop {
        current_idx -= 1;
        // println!("{} {:?}", &current_idx, &values);
        if current_idx < 0 {
            break;
        }
        let (pos, full) = values
            .iter()
            .enumerate()
            .find(|(_, x)| match x {
                Type::Full(_, idx) => *idx == current_idx as u32,
                Type::Empty(_) => false,
            })
            .expect("Must be present");

        if let Type::Full(full_len, full_idx) = full.clone() {
            let empty = values.iter().enumerate().find(|(p, x)| {
                if *p >= pos {
                    return false;
                }
                match x {
                    Type::Empty(empty_len) => *empty_len >= full_len,
                    Type::Full(_, _) => false,
                }
            });

            if empty.is_none() {
                continue;
            }

            if let (empty_pos, Type::Empty(empty_len)) = empty.unwrap() {
                let elen = *empty_len;
                values[pos] = Type::Empty(full_len);
                values.remove(empty_pos);
                values.insert(empty_pos, Type::Full(full_len, full_idx));
                if elen > full_len {
                    values.insert(empty_pos + 1, Type::Empty(elen - full_len));
                }
            }
        }
    }

    let mut acc = 0u64;
    let mut current_idx = 0u64;

    for val in values {
        match val {
            Type::Full(len, idx) => {
                for _ in 0..len {
                    acc += current_idx * idx as u64;
                    current_idx += 1;
                }
            }
            Type::Empty(len) => {
                for _ in 0..len {
                    current_idx += 1;
                }
            }
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
