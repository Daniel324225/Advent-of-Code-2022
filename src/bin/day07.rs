use std::{collections::HashMap, iter::Peekable};

#[derive(Default)]
struct Folder {
    folders: HashMap<String, Folder>,
    files: HashMap<String, usize>,
}

enum Status {
    Up,
    Root,
    Finish,
}

fn parse<'a, I: Iterator<Item = &'a str>>(
    current_folder: &mut Folder,
    lines: &mut Peekable<I>,
) -> Result<Status, String> {
    while let Some(line) = lines.next() {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", ".."] => return Ok(Status::Up),
            ["$", "cd", "/"] => return Ok(Status::Root),
            ["$", "cd", folder] => {
                let status = parse(
                    current_folder
                        .folders
                        .entry(String::from(*folder))
                        .or_default(),
                    lines,
                );
                if let Ok(Status::Up) = status {
                    continue;
                } else {
                    return status;
                }
            }
            ["$", "ls"] => {
                while let Some(line) = lines.peek() {
                    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                        ["$", ..] => break,
                        ["dir", folder] => {
                            current_folder
                                .folders
                                .entry(folder.to_string())
                                .or_default();
                        }
                        [size, filename] => {
                            current_folder.files.insert(
                                filename.to_string(),
                                size.parse().map_err(|_| "Invalid size".to_string())?,
                            );
                        }
                        _ => return Err("Invalid ls result".to_string()),
                    }
                    lines.next();
                }
            }
            _ => return Err("Invalid command".to_string()),
        };
    }
    Ok(Status::Finish)
}

fn part1(input: &str) -> Result<String, String> {
    let mut lines = input.lines().peekable();

    let mut root = Folder::default();

    while let Status::Root = parse(&mut root, &mut lines)? {
        continue;
    }

    fn get_size(folder: &Folder) -> (usize, usize) {
        let (mut size, mut sum) = folder
            .folders
            .values()
            .map(get_size)
            .fold((0, 0), |(total_size, total_sum), (size, sum)| {
                (total_size + size, total_sum + sum)
            });
        size += folder.files.values().sum::<usize>();
        if size <= 100000 {
            sum += size
        }
        (size, sum)
    }

    Ok(get_size(&root).1.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut lines = input.lines().peekable();

    let mut root = Folder::default();

    while let Status::Root = parse(&mut root, &mut lines)? {
        continue;
    }

    let mut sizes = Vec::new();
    fn get_sizes(folder: &Folder, sizes: &mut Vec<usize>) -> usize {
        let size = folder
            .folders
            .values()
            .map(|folder| get_sizes(folder, sizes))
            .sum::<usize>()
            + folder.files.values().sum::<usize>();
        sizes.push(size);
        size
    }
    let total_size = get_sizes(&root, &mut sizes);

    sizes.sort();

    let index = sizes.partition_point(|&size| 70000000 - total_size + size < 30000000);

    Ok(sizes
        .get(index)
        .ok_or_else(|| "Not found".to_string())?
        .to_string())
}

advent_of_code::aoc_main!(7);
