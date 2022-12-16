use std::collections::HashSet;

struct Cave {
    cave: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    max_y: i32,
}

fn parse(input: &str) -> Result<Cave, String> {
    let mut cave = HashSet::new();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    let mut update_border = |(x, y)| {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    };

    for mut line in input.lines().map(|line| {
        line.split("->").map(|pair| {
            pair.split_once(',')
                .and_then(|(a, b)| -> Option<(i32, i32)> {
                    Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
                })
                .ok_or_else(|| "Invalid Input".to_string())
        })
    }) {
        let mut last = line.next().ok_or_else(|| "Empty line".to_string())??;
        update_border(last);
        for point in line {
            let point = point?;
            update_border(point);
            let insert = |point| {
                cave.insert(point);
            };
            match point {
                (x, y) if x == last.0 => (y.min(last.1)..=y.max(last.1))
                    .map(|a| (x, a))
                    .for_each(insert),
                (x, y) if y == last.1 => (x.min(last.0)..=x.max(last.0))
                    .map(|a| (a, y))
                    .for_each(insert),
                _ => return Err("Invalid line".to_string()),
            };
            last = point;
        }
    }

    Ok(Cave {
        cave,
        min_x,
        max_x,
        max_y,
    })
}

fn put_sand(cave: &mut Cave) -> bool {
    let (mut x, mut y) = (500, 0);
    while x >= cave.min_x && x <= cave.max_x && y <= cave.max_y {
        if let Some(point) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
            .into_iter()
            .find(|point| !cave.cave.contains(point))
        {
            x = point.0;
            y = point.1;
        } else {
            cave.cave.insert((x, y));
            return true;
        }
    }
    false
}

fn put_sand_p2(cave: &mut Cave) -> bool {
    let floor = cave.max_y + 1;
    let (mut x, mut y) = (500, 0);

    if cave.cave.contains(&(x, y)) {
        return false;
    }

    while let Some(point) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
        .into_iter()
        .find(|point| !cave.cave.contains(point))
    {
        x = point.0;
        y = point.1;

        if y == floor {
            break;
        }
    }

    cave.cave.insert((x, y));
    true
}

fn part1(input: &str) -> Result<String, String> {
    let mut cave = parse(input)?;
    let walls = cave.cave.len();
    while put_sand(&mut cave) {
        continue;
    }

    Ok((cave.cave.len() - walls).to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut cave = parse(input)?;
    let walls = cave.cave.len();
    while put_sand_p2(&mut cave) {
        continue;
    }

    Ok((cave.cave.len() - walls).to_string())
}

advent_of_code::aoc_main!(14);
