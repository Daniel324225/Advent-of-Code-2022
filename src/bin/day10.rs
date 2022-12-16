const WIDTH: i32 = 40;

fn part1(input: &str) -> Result<String, String> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next()? {
                "noop" => Some((0, 1)),
                "addx" => Some((parts.next()?.parse().ok()?, 2)),
                _ => None,
            }
        })
        .zip(1..)
        .scan(1, |x, (pair, line_number)| {
            Some(match pair {
                Some((value, cycles)) => {
                    std::iter::repeat(Ok(std::mem::replace(x, *x + value))).take(cycles)
                }
                None => {
                    std::iter::repeat(Err(format!("Invalid instruction on line {line_number}")))
                        .take(1)
                }
            })
        })
        .flatten()
        .zip(1..)
        .try_fold(0, |sum, (value, cycle)| {
            value.map(|value| {
                if cycle % WIDTH == 20 {
                    sum + value * cycle
                } else {
                    sum
                }
            })
        })
        .map(|sum| sum.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next()? {
                "noop" => Some((0, 1)),
                "addx" => Some((parts.next()?.parse().ok()?, 2)),
                _ => None,
            }
        })
        .zip(1..)
        .scan(1, |x, (pair, line_number)| {
            Some(match pair {
                Some((value, cycles)) => {
                    std::iter::repeat(Ok(std::mem::replace(x, *x + value))).take(cycles)
                }
                None => {
                    std::iter::repeat(Err(format!("Invalid instruction on line {line_number}")))
                        .take(1)
                }
            })
        })
        .flatten()
        .zip(0i32..)
        .map(|(sprite_position, pixel_position)| {
            sprite_position.map(|sprite_position| {
                (
                    pixel_position,
                    if (pixel_position % WIDTH).abs_diff(sprite_position) <= 1 {
                        "#"
                    } else {
                        " "
                    },
                )
            })
        })
        .try_fold(String::with_capacity(250), |mut string, pixel| {
            pixel.map(|(position, value)| {
                if position % WIDTH == 0 {
                    string.push('\n');
                }
                string + value
            })
        })
}

advent_of_code::aoc_main!(10);
