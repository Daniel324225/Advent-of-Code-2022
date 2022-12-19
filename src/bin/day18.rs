use std::collections::{HashSet, VecDeque};

fn part1(input: &str) -> Result<String, String> {
    let mut touches = 0;
    let mut cubes = HashSet::new();
    for cube in input.lines().map(|line| -> Option<[i32; 3]> {
        Some(
            line.split(',')
                .map(|a| a.trim().parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .ok()?
                .try_into()
                .ok()?,
        )
    }) {
        let cube = cube.ok_or_else(|| "Invalid input".to_string())?;

        for idx in 0..3 {
            for d in [-1, 1] {
                let mut neighbor = cube;
                neighbor[idx] += d;
                if cubes.contains(&neighbor) {
                    touches += 1;
                }
            }
        }
        cubes.insert(cube);
    }
    Ok((cubes.len() as i32 * 6 - touches * 2).to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut min = [i32::MAX; 3];
    let mut max = [i32::MIN; 3];
    let mut touches = 0;
    let mut cubes = HashSet::new();
    for cube in input.lines().map(|line| -> Option<[i32; 3]> {
        Some(
            line.split(',')
                .map(|a| a.trim().parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .ok()?
                .try_into()
                .ok()?,
        )
    }) {
        let cube = cube.ok_or_else(|| "Invalid input".to_string())?;

        for idx in 0..3 {
            min[idx] = min[idx].min(cube[idx]);
            max[idx] = max[idx].max(cube[idx]);
        }
        cubes.insert(cube);
    }

    min.iter_mut().for_each(|a| *a -= 1);
    max.iter_mut().for_each(|a| *a += 1);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(min);
    visited.insert(min);

    while let Some(current) = queue.pop_front() {
        for idx in 0..3 {
            for d in [-1, 1] {
                let mut neighbor = current;
                neighbor[idx] += d;

                if neighbor
                    .iter()
                    .enumerate()
                    .all(|(idx, value)| (min[idx]..=max[idx]).contains(value))
                    && !visited.contains(&neighbor)
                {
                    if cubes.contains(&neighbor) {
                        touches += 1;
                    } else {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    Ok(touches.to_string())
}

advent_of_code::aoc_main!(18);
