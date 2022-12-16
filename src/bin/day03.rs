fn priority(c: &u8) -> usize {
    (match *c {
        c @ b'a'..=b'z' => c - b'a' + 1,
        c @ b'A'..=b'Z' => c - b'A' + 27,
        _ => 0,
    }) as usize
}

fn part1(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|line| {
            let mut table = [false; 53];
            let line = line.as_bytes();
            let mid = line.len() / 2;
            for p in line[..mid].iter().map(priority) {
                table[p] = true;
            }
            for p in line[mid..].iter().map(priority) {
                if table[p] {
                    return p;
                }
            }
            0
        })
        .sum::<usize>()
        .to_string())
}

fn part2(input: &str) -> Result<String, String> {
    use std::collections::HashSet;
    struct State {
        items: i32,
        intersection: HashSet<usize>,
        sum: usize,
    }

    let group_size = 3;

    let all: HashSet<_> = (1..=52usize).collect();

    Ok(input
        .lines()
        .fold(
            State {
                items: 0,
                intersection: all.clone(),
                sum: 0,
            },
            |mut state, line| {
                let State {
                    items,
                    intersection,
                    sum,
                } = &mut state;

                *intersection = line
                    .as_bytes()
                    .iter()
                    .map(priority)
                    .filter(|x| intersection.contains(x))
                    .collect();

                *items += 1;

                if *items == group_size {
                    *sum += intersection.iter().next().unwrap_or(&0);
                    *items = 0;
                    *intersection = all.clone()
                }

                state
            },
        )
        .sum
        .to_string())
}

advent_of_code::aoc_main!(3);
