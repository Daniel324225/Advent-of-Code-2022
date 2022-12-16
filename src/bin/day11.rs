use regex::Regex;
use std::collections::VecDeque;
#[derive(Clone)]
enum Arg {
    Old,
    Int(i64),
}
enum Operation {
    Mul,
    Add,
}

struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    args: [Arg; 2],
    test: i64,
    if_true: usize,
    if_false: usize,
    count: i64,
}

fn parse(input: &str) -> Option<Vec<Monkey>> {
    let mut monkeys = Vec::new();

    let regex = Regex::new(concat!(
        r#"Monkey (\d+):\s*"#,
        r#"Starting items: ([0-9, ]+)\s*"#,
        r#"Operation: new = (old|\d+) ([+*]) (old|\d+)\s*"#,
        r#"Test: divisible by (\d+)\s*"#,
        r#"If true: throw to monkey (\d+)\s*"#,
        r#"If false: throw to monkey (\d+)\s*"#,
    ))
    .unwrap();

    for monkey in regex.captures_iter(input) {
        if monkey.get(1)?.as_str().parse::<usize>().ok()? != monkeys.len() {
            return None;
        }

        monkeys.push(Monkey {
            items: monkey
                .get(2)?
                .as_str()
                .split(',')
                .map(|item| item.trim().parse().ok())
                .collect::<Option<VecDeque<_>>>()?,
            operation: match monkey.get(4)?.as_str() {
                "*" => Some(Operation::Mul),
                "+" => Some(Operation::Add),
                _ => None,
            }?,
            args: {
                let parse_arg = |capture: Option<regex::Match>| {
                    let str = capture?.as_str();
                    Some(if str == "old" {
                        Arg::Old
                    } else {
                        Arg::Int(str.parse().ok()?)
                    })
                };
                [parse_arg(monkey.get(3))?, parse_arg(monkey.get(5))?]
            },
            test: monkey.get(6)?.as_str().parse().ok()?,
            if_true: monkey.get(7)?.as_str().parse().ok()?,
            if_false: monkey.get(8)?.as_str().parse().ok()?,
            count: 0,
        })
    }

    Some(monkeys)
}

fn simulate(monkeys: &mut Vec<Monkey>, div: i64, rounds: i32) {
    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test)
        .fold(1, |a, b| a * b);
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            while let Some(item) = monkeys[index].items.pop_front() {
                monkeys[index].count += 1;
                let monkey = &monkeys[index];

                let get_arg = |arg| match arg {
                    Arg::Old => item,
                    Arg::Int(value) => value,
                };
                let args = monkey.args.iter().cloned().map(get_arg);
                let item = match monkey.operation {
                    Operation::Add => args.sum(),
                    Operation::Mul => args.fold(1, |a, b| a * b),
                } / div;
                let item = item % modulo;
                let throw_to = if item % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                monkeys[throw_to].items.push_back(item);
            }
        }
    }
}

fn part1(input: &str) -> Result<String, String> {
    let mut monkeys = parse(input).ok_or("Invalid input".to_string())?;
    simulate(&mut monkeys, 3, 20);

    let mut counts: Vec<_> = monkeys.iter().map(|monkey| monkey.count).collect();
    counts.sort_by_key(|x| std::cmp::Reverse(*x));

    Ok(counts.iter().take(2).fold(1, |a, b| a * b).to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut monkeys = parse(input).ok_or("Invalid input".to_string())?;
    simulate(&mut monkeys, 1, 10000);

    let mut counts: Vec<_> = monkeys.iter().map(|monkey| monkey.count).collect();
    counts.sort_by_key(|x| std::cmp::Reverse(*x));

    Ok(counts.iter().take(2).fold(1, |a, b| a * b).to_string())
}

advent_of_code::aoc_main!(11);
