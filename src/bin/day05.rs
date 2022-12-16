fn parse_state<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Option<Vec<Vec<u8>>> {
    let state: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();

    let size: usize = state
        .last()?
        .split_whitespace()
        .map(|n| n.parse().ok())
        .try_fold(0, |previous, current| {
            if current? == previous + 1 {
                current
            } else {
                None
            }
        })?;

    (0..size)
        .map(|index| {
            let mut chars = state
                .iter()
                .rev()
                .skip(1)
                .map(|line| *line.as_bytes().get(1 + index * 4).unwrap_or(&b' '));
            let vec = chars.by_ref().take_while(|&c| c != b' ').collect();
            if chars.all(|c| c == b' ') {
                Some(vec)
            } else {
                None
            }
        })
        .collect()
}

fn parse_move(line: &str) -> Option<[usize; 3]> {
    let parts: [&str; 6] = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .ok()?;

    if parts.iter().step_by(2).eq(["move", "from", "to"].iter()) {
        parts
            .iter()
            .skip(1)
            .step_by(2)
            .map(|num| num.parse().ok())
            .collect::<Option<Vec<_>>>()
            .and_then(|nums| nums.try_into().ok())
    } else {
        None
    }
}

fn part1(input: &str) -> Result<String, String> {
    let mut lines = input.lines();

    let mut state = parse_state(&mut lines).ok_or("Invalid initial state")?;

    let moves = lines.map(parse_move);

    for (line_idx, m) in moves.enumerate() {
        let [amount, from, to] =
            m.ok_or_else(|| format!("Invalid move format on line {}", line_idx))?;
        let stack_index_err = |index| format!("Invalid stack index {} on line {}", index, line_idx);
        for _ in 0..amount {
            let element = state
                .get_mut(from - 1)
                .ok_or_else(|| stack_index_err(from))?
                .pop()
                .ok_or_else(|| format!("Invalid move on line {}. Amount too big", line_idx))?;

            state
                .get_mut(to - 1)
                .ok_or_else(|| stack_index_err(to))?
                .push(element);
        }
    }

    Ok(state
        .iter()
        .flat_map(|stack| stack.last())
        .map(|&c| c as char)
        .collect())
}

fn part2(input: &str) -> Result<String, String> {
    let mut lines = input.lines();

    let mut state = parse_state(&mut lines).ok_or("Invalid initial state")?;

    let moves = lines.map(parse_move);

    for (line_idx, m) in moves.enumerate() {
        let [amount, from, to] =
            m.ok_or_else(|| format!("Invalid move format on line {}", line_idx))?;

        let stack_index_err = |index| format!("Invalid stack index {} on line {}", index, line_idx);

        let from = state
            .get_mut(from - 1)
            .ok_or_else(|| stack_index_err(from))?;

        if amount > from.len() {
            return Err(format!("Invalid move on line {}. Amount too big", line_idx));
        }

        let crates = from.split_off(from.len() - amount);

        state
            .get_mut(to - 1)
            .ok_or_else(|| stack_index_err(to))?
            .extend_from_slice(&crates);
    }

    Ok(state
        .iter()
        .flat_map(|stack| stack.last())
        .map(|&c| c as char)
        .collect())
}

advent_of_code::aoc_main!(5);
