fn find_consecutive_unique(text: &str, len: usize) -> Result<String, String> {
    let mut table = [false; 256];
    let mut unique_count = 0;
    for (index, c) in text.bytes().enumerate() {
        if table[c as usize] {
            let previous_occurrence = (0..index)
                .rev()
                .find(|&idx| text.as_bytes()[idx] == c)
                .unwrap();

            let to_remove = (index - unique_count)..previous_occurrence;

            unique_count -= to_remove.len();
            for &c in text.as_bytes()[to_remove].iter() {
                table[c as usize] = false;
            }
        } else {
            table[c as usize] = true;
            unique_count += 1;
            if unique_count == len {
                return Ok((index + 1).to_string());
            }
        }
    }
    Err("Not found".to_string())
}

fn part1(input: &str) -> Result<String, String> {
    find_consecutive_unique(input, 4)
}

fn part2(input: &str) -> Result<String, String> {
    find_consecutive_unique(input, 14)
}

advent_of_code::aoc_main!(6);
