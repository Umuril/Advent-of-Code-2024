use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list1, IResult};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Game {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

fn parse_part_one(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Button A: X+")(input)?;
    let (input, x) = u64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = u64(input)?;
    let button_a = Point { x, y };

    let (input, _) = tag("\nButton B: X+")(input)?;
    let (input, x) = u64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = u64(input)?;
    let button_b = Point { x, y };

    let (input, _) = tag("\nPrize: X=")(input)?;
    let (input, x) = u64(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = u64(input)?;
    let prize = Point { x, y };

    Ok((
        input,
        Game {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn get_game_info(game: &Game, pressed: (u64, u64)) -> Option<u64> {
    if game.button_a.x * pressed.0 + game.button_b.x * pressed.1 != game.prize.x {
        return None;
    }
    if game.button_a.y * pressed.0 + game.button_b.y * pressed.1 != game.prize.y {
        return None;
    }

    Some(pressed.0 * 3 + pressed.1)
}

pub fn part_one(input: &str) -> Option<String> {
    let data = separated_list1(tag("\n\n"), parse_part_one)(input.trim())
        .expect("Correct input format")
        .1;

    let mut total_tokens = 0;
    for game in data {
        let mut min_tokens = None;
        for i in 0..=100 {
            for j in 0..=100 {
                if let Some(tokens) = get_game_info(&game, (i, j)) {
                    if min_tokens.is_none() || min_tokens.is_some_and(|m| tokens < m) {
                        min_tokens = Some(tokens);
                    }
                }
            }
        }
        if let Some(m) = min_tokens {
            total_tokens += m;
        }
    }

    Some(total_tokens.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut data = separated_list1(tag("\n\n"), parse_part_one)(input.trim())
        .expect("Correct input format")
        .1;

    data.iter_mut().for_each(|g| {
        g.prize.x += 10000000000000;
        g.prize.y += 10000000000000;
    });

    let mut total_tokens = 0;
    for game in data {
        let (ax, ay, bx, by, px, py) = (
            game.button_a.x as i64,
            game.button_a.y as i64,
            game.button_b.x as i64,
            game.button_b.y as i64,
            game.prize.x as i64,
            game.prize.y as i64,
        );
        let num = bx * ax * py - bx * ay * px;
        let div = by * ax - ay * bx;
        // println!("{num} {div} {game:#?}");

        let meet = num / div;
        let button_b = meet / bx;
        let button_a = (px - button_b * bx) / ax;
        // println!("{button_a} {button_b}");

        if button_a * ay + button_b * by == py {
            total_tokens += button_a as u64 * 3 + button_b as u64;
        }
    }

    Some(total_tokens.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("480".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("875318608908".to_string()));
    }
}
