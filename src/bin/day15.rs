#[derive(Clone, Copy)]
struct Pair {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

fn parse(input: &str) -> Vec<Pair> {
    let regex = regex::Regex::new(
        r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#,
    )
    .unwrap();

    let mut vec = Vec::new();

    for m in regex.captures_iter(input) {
        vec.push(Pair {
            sensor: (
                m.get(1).unwrap().as_str().parse().unwrap(),
                m.get(2).unwrap().as_str().parse().unwrap(),
            ),
            beacon: (
                m.get(3).unwrap().as_str().parse().unwrap(),
                m.get(4).unwrap().as_str().parse().unwrap(),
            ),
        })
    }

    vec
}

fn part1(input: &str) -> Result<String, String> {
    let mut intervals = Vec::new();
    let mut beacons = std::collections::HashSet::new();
    for Pair {
        sensor: (sensor_x, sensor_y),
        beacon: (beacon_x, beacon_y),
    } in parse(input).into_iter()
    {
        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        let d = distance - (sensor_y - 2000000).abs();

        //println!("Sensor: ({sensor_x}, {sensor_y})\nBeacon: ({beacon_x}, {beacon_y})\n{distance}\nd = {d}");

        if d < 0 {
            continue;
        }

        intervals.push((sensor_x - d, sensor_x + d + 1));

        //println!("Interval: {:?}\n-----------", intervals.last().unwrap());

        if beacon_y == 2000000 {
            beacons.insert(beacon_x);
        }
    }
    //println!("=============");

    intervals.sort();

    let mut sum = 0;

    let mut last = i32::MIN;
    for (a, b) in intervals.into_iter() {
        //println!("Interval: ({a}, {b})\n");

        let a = a.max(last);
        sum += (b - a).max(0);
        last = last.max(b);

        //println!("Sum: {sum}\n------");
    }

    Ok((sum - beacons.len() as i32).to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let input = parse(input);

    let max = 20;

    for y in 0..=4000000 {
        let mut intervals = Vec::new();
        for Pair {
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        } in input.iter().copied()
        {
            let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            let d = distance - (sensor_y - y).abs();

            if d < 0 {
                continue;
            }

            intervals.push((sensor_x - d, sensor_x + d + 1));
        }

        intervals.sort();

        let mut last = 0;
        for (a, b) in intervals.into_iter() {
            let a = a.max(last);

            if a != last {
                return Ok((last as i64 * 4000000 + y as i64).to_string());
            }

            last = last.max(b);
        }

        if last <= max {
            return Ok((last as i64 * 4000000 + y as i64).to_string());
        }
    }
    Err("Not found".to_string())
}

advent_of_code::aoc_main!(15);
