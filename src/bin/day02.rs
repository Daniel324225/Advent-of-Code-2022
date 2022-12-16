fn part1(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|line| {
            if let [a, b' ', b] = line.as_bytes() {
                let your_move = (b - b'X') as i32;
                let opponent_move = (a - b'A') as i32;
                let result = (your_move - opponent_move + 1).rem_euclid(3);
                result * 3 + your_move + 1
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string())
}

fn part2(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|line| {
            if let [a, b' ', b] = line.as_bytes() {
                let result = (*b - b'X') as i32;
                let opponent_move = (*a - b'A') as i32;
                result * 3 + (opponent_move + result + 2) % 3 + 1
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string())
}

advent_of_code::aoc_main!(2);
