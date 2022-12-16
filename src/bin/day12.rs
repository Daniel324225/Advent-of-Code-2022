type Map = Vec<Vec<(u8, Option<i32>)>>;

struct Input {
    start: (usize, usize),
    end: (usize, usize),
    map: Map,
}

fn parse(input: &str) -> Result<Input, String> {
    let mut start = Err("No start position");
    let mut end = Err("No end position");

    let map: Map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => {
                            if start.is_ok() {
                                return Err("More then one start position".to_string());
                            }
                            start = Ok((x, y));
                            Ok(b'a')
                        }
                        'E' => {
                            if end.is_ok() {
                                return Err("More then one end position".to_string());
                            }
                            end = Ok((x, y));
                            Ok(b'z')
                        }
                        c @ 'a'..='z' => Ok(c as u8),
                        _ => Err("Invalid character".to_string()),
                    }
                    .map(|height| (height, None))
                })
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    let width = map.first().ok_or_else(|| "Empty input".to_string())?.len();

    if !map.iter().map(|line| line.len()).all(|len| len == width) {
        return Err("Not a rectangle".to_string());
    }
    let start = start?;
    let end = end?;
    Ok(Input { start, end, map })
}

fn bfs(map: &mut Map, start: (usize, usize), test: impl Fn(u8, u8) -> bool) {
    let len = map.len() as i32;
    let width = map.first().unwrap().len() as i32;

    let mut queue = std::collections::VecDeque::new();
    map[start.1][start.0].1 = Some(0);
    queue.push_back((start.0 as i32, start.1 as i32));

    while let Some((x, y)) = queue.pop_front() {
        let (height, distance) = map[y as usize][x as usize];
        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if !((0..len).contains(&y) && (0..width).contains(&x)) {
                continue;
            }
            if let Some(neighbor) = map
                .get_mut(y as usize)
                .and_then(|line| line.get_mut(x as usize))
            {
                if test(height, neighbor.0) && neighbor.1.is_none() {
                    neighbor.1 = distance.map(|d| d + 1);
                    queue.push_back((x, y));
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<String, String> {
    let Input {
        start,
        end,
        mut map,
    } = parse(input)?;

    bfs(&mut map, start, |from, to| to <= from + 1);

    map[end.1][end.0]
        .1
        .map(|d| d.to_string())
        .ok_or_else(|| "End unreachable".to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let Input {
        start: _,
        end,
        mut map,
    } = parse(input)?;

    bfs(&mut map, end, |to, from| to <= from + 1);

    map.iter()
        .flatten()
        .flat_map(|(h, distance)| if *h == b'a' { *distance } else { None })
        .min()
        .ok_or_else(|| "Unreachable".to_string())
        .map(|d| d.to_string())
}

advent_of_code::aoc_main!(12);
