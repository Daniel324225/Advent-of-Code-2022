fn split(line: &str, c: char) -> Option<[&str; 2]> {
    let mut parts = line.split(c);
    let pair = [parts.next()?, parts.next()?];
    if parts.next().is_none() {
        Some(pair)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Vec<Result<[i32; 4], String>> {
    input
        .lines()
        .map(|line| {
            split(line, ',')
                .and_then(|pair| Some([split(pair[0], '-')?, split(pair[1], '-')?].concat()))
        })
        .enumerate()
        .map(|(index, nums)| -> Result<[i32; 4], _> {
            nums.and_then(|vec| {
                vec.iter()
                    .map(|num| num.parse::<i32>().ok())
                    .collect::<Option<Vec<i32>>>()
            })
            .and_then(|vec| vec.try_into().ok())
            .ok_or_else(|| format!("Parse error on line {index}"))
        })
        .collect()
}

fn part1(input: &str) -> Result<String, String> {
    parse_input(input).into_iter()
        .try_fold(0, |state, current| {
            current.map(|[begin1, end1, begin2, end2]| if (begin1 <= begin2 && end2 <= end1) || (begin2 <= begin1 && end1 <= end2) {1} else {0} + state)
        }).map(|answer| answer.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    parse_input(input).into_iter()
        .try_fold(0, |state, current| {
            current.map(|[begin1, end1, begin2, end2]| if begin1.max(begin2) <= end1.min(end2) {1} else {0} + state)
        }).map(|answer| answer.to_string())
}

advent_of_code::aoc_main!(4);
