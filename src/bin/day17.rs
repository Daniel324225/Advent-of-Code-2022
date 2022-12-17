static ROCKS: [[[u8; 4]; 4]; 5] = [
    [
        *b"####", //
        *b"    ", //
        *b"    ", //
        *b"    ", //
    ],
    [
        *b" #  ", //
        *b"### ", //
        *b" #  ", //
        *b"    ", //
    ],
    [
        *b"### ", //
        *b"  # ", //
        *b"  # ", //
        *b"    ", //
    ],
    [
        *b"#   ", //
        *b"#   ", //
        *b"#   ", //
        *b"#   ", //
    ],
    [
        *b"##  ", //
        *b"##  ", //
        *b"    ", //
        *b"    ", //
    ],
];

fn collision(rock: &[[u8; 4]; 4], x: usize, y: usize, board: &Vec<[u8; 9]>) -> bool {
    for dy in 0..4 {
        for dx in 0..4 {
            if rock[dy][dx] == b'#' && board[y + dy][x + dx] == b'#' {
                return true;
            }
        }
    }
    false
}

fn part1(input: &str) -> Result<String, String> {
    let input: Vec<isize> = input
        .chars()
        .map(|c| match c {
            '<' => Ok(-1isize),
            '>' => Ok(1isize),
            _ => Err("Invalid input".to_string()),
        })
        .collect::<Result<_, _>>()?;

    let mut wind = input.iter().copied().cycle();

    let mut moves = Vec::new();

    let mut board: Vec<[u8; 9]> = vec![[b'#'; 9]; 1];
    let mut max = 0;

    for (kind, rock) in ROCKS.iter().enumerate().cycle().take(2022) {
        board.resize(max + 8, *b"#       #");
        let start = max + 4;
        let mut y = start;
        let mut x = 3isize;
        loop {
            let new_x = x + wind.next().unwrap();
            if !collision(rock, new_x as usize, y, &board) {
                x = new_x;
            }
            let new_y = y - 1;
            if collision(rock, x as usize, new_y, &board) {
                break;
            } else {
                y = new_y;
            }
        }

        for dy in 0..4 {
            for dx in 0..4 {
                if rock[dy][dx] == b'#' {
                    board[y + dy][x as usize + dx] = b'#';
                    max = max.max(y + dy);
                }
            }
        }

        moves.push((kind, x, start - y));
    }

    Ok(max.to_string())
}

fn part2(_: &str) -> Result<String, String> {
    Ok("".to_string())
}

advent_of_code::aoc_main!(17);
