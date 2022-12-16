fn part1(input: &str) -> Result<String, String> {
    let mut trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|c| (false, c)).collect())
        .collect();

    let length = trees.len();
    let width = trees
        .first()
        .and_then(|first| {
            if trees.iter().all(|line| line.len() == first.len()) {
                Some(first.len())
            } else {
                None
            }
        })
        .ok_or_else(|| "Input is not a rectangle".to_string())?;

    fn handle_line(
        indexes: impl Iterator<Item = (usize, usize)>,
        trees: &mut Vec<Vec<(bool, u8)>>,
    ) {
        let mut max_height = 0;
        for (x, y) in indexes {
            let tree = trees.get_mut(y).and_then(|line| line.get_mut(x));

            if let Some((visible, height)) = tree {
                if *height > max_height {
                    *visible = true;
                    max_height = *height;
                    if *height == b'9' {
                        return;
                    }
                }
            }
        }
    }

    for y in 0..length {
        handle_line((0..width).map(|x| (x, y)), &mut trees);
        handle_line((0..width).rev().map(|x| (x, y)), &mut trees);
    }

    for x in 0..width {
        handle_line((0..length).map(|y| (x, y)), &mut trees);
        handle_line((0..length).rev().map(|y| (x, y)), &mut trees);
    }

    Ok(trees
        .iter()
        .flatten()
        .map(|tree| (*tree).0 as i32)
        .sum::<i32>()
        .to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|c| (1, c)).collect())
        .collect();

    let length = trees.len();
    let width = trees
        .first()
        .and_then(|first| {
            if trees.iter().all(|line| line.len() == first.len()) {
                Some(first.len())
            } else {
                None
            }
        })
        .ok_or_else(|| "Input is not a rectangle".to_string())?;

    fn handle_line(indexes: impl Iterator<Item = (usize, usize)>, trees: &mut Vec<Vec<(i32, u8)>>) {
        let mut previous = [0; 10];
        for (position, (x, y)) in indexes.enumerate() {
            let (score, height) = trees.get_mut(y).and_then(|line| line.get_mut(x)).unwrap();
            let height = (*height - b'0') as usize;

            *score *= position as i32 - *previous[height..].iter().max().unwrap() as i32;
            previous[height] = position;
        }
    }

    for y in 0..length {
        handle_line((0..width).map(|x| (x, y)), &mut trees);
        handle_line((0..width).rev().map(|x| (x, y)), &mut trees);
    }

    for x in 0..width {
        handle_line((0..length).map(|y| (x, y)), &mut trees);
        handle_line((0..length).rev().map(|y| (x, y)), &mut trees);
    }

    Ok(trees
        .iter()
        .flatten()
        .map(|(score, _)| *score)
        .max()
        .unwrap_or_default()
        .to_string())
}

advent_of_code::aoc_main!(8);
