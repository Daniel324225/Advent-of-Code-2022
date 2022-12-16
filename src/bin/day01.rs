fn part1(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let sums = std::iter::from_fn(|| {
        let mut sum: i32 = lines.next()?.parse().ok()?; //stop iterating if there is no next line
        while let Some(x) = lines.next().and_then(|line| line.parse::<i32>().ok()) {
            //while next line is a valid integer add it to sum
            sum += x;
        }
        Some(sum)
    });

    Ok(sums.max().unwrap_or_default().to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut lines = input.lines();
    let sums = std::iter::from_fn(|| {
        let mut sum: i32 = lines.next()?.parse().ok()?; //stop iterating if there is no next line
        while let Some(x) = lines.next().and_then(|line| line.parse::<i32>().ok()) {
            //while next line is a valid integer add it to sum
            sum += x;
        }
        Some(sum)
    });

    let insert_sorted = |slice: &mut [_], value| {
        let index = slice.partition_point(|x| *x > value);
        if index < slice.len() {
            *slice.last_mut().unwrap() = value;
            slice[index..].rotate_right(1);
        }
    };

    Ok(sums
        .fold([0; 3], |mut top, current| {
            insert_sorted(&mut top, current);
            top
        })
        .iter()
        .sum::<i32>()
        .to_string())
}

advent_of_code::aoc_main!(1);
