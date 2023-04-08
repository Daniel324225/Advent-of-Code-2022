fn parse(input: &str) -> Result<Vec<(i32, i64)>, String> {
    input
        .lines()
        .map(|line| line.parse().map_err(|_| "Invalid input".to_string()))
        .scan(0, |count, num| {
            *count += 1;
            Some(num.map(|num| (*count, num)))
        })
        .collect::<Result<_, _>>()
}

fn mix(vec: &mut Vec<(i32, i64)>) {
    let len = vec.len();

    for index in 1..=len as i32 {
        let index = vec.iter().position(|(a, _)| *a == index).unwrap();
        let new_idx = index as i64 + vec[index].1.rem_euclid(len as i64 - 1);
        let new_idx = (new_idx + (new_idx >= len as i64) as i64 - (new_idx <= 0) as i64)
            .rem_euclid(len as i64) as usize;

        if new_idx >= index {
            vec[index..=new_idx].rotate_left(1);
        } else {
            vec[new_idx..=index].rotate_right(1);
        }
    }
}

fn part1(input: &str) -> Result<String, String> {
    let mut nums = parse(input)?;

    mix(&mut nums);

    let zero_pos = nums
        .iter()
        .position(|(_, a)| *a == 0)
        .ok_or_else(|| "0 not found".to_string())?;

    Ok(nums
        .iter()
        .cycle()
        .skip(zero_pos)
        .step_by(1000)
        .take(4)
        .map(|(_, a)| *a)
        .sum::<i64>()
        .to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut nums = parse(input)?;

    for num in nums.iter_mut() {
        num.1 *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut nums);
    }

    let zero_pos = nums
        .iter()
        .position(|(_, a)| *a == 0)
        .ok_or_else(|| "0 not found".to_string())?;

    Ok(nums
        .iter()
        .cycle()
        .skip(zero_pos)
        .step_by(1000)
        .take(4)
        .inspect(|a| println!("{a:?}"))
        .map(|(_, a)| *a)
        .sum::<i64>()
        .to_string())
}

advent_of_code::aoc_main!(20);
