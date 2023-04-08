use std::collections::HashMap;

enum Op {
    Add,
    Div,
    Sub,
    Mul,
}

enum Job<'a> {
    Number(i64),
    Expr(&'a str, Op, &'a str),
}

fn eval(job: &Job, map: &HashMap<&str, Job>) -> Result<i64, String> {
    Ok(match job {
        Job::Number(number) => *number,
        Job::Expr(arg1, op, arg2) => {
            let arg1 = map
                .get(arg1)
                .ok_or_else(|| "Unknown name in expression".to_string())?;
            let arg2 = map
                .get(arg2)
                .ok_or_else(|| "Unknown name in expression".to_string())?;

            let arg1 = eval(arg1, map)?;
            let arg2 = eval(arg2, map)?;

            match op {
                Op::Add => arg1 + arg2,
                Op::Sub => arg1 - arg2,
                Op::Mul => arg1 * arg2,
                Op::Div => arg1 / arg2,
            }
        }
    })
}

fn parse(input: &str) -> Option<HashMap<&str, Job>> {
    input
        .lines()
        .map(|line| {
            line.split_once(':').and_then(|(name, job)| {
                Some((
                    name,
                    match job.split_whitespace().collect::<Vec<_>>()[..] {
                        [number] => Job::Number(number.parse().ok()?),
                        [arg1, op, arg2] => Job::Expr(
                            arg1,
                            match op {
                                "+" => Op::Add,
                                "-" => Op::Sub,
                                "/" => Op::Div,
                                "*" => Op::Mul,
                                _ => return None,
                            },
                            arg2,
                        ),
                        _ => return None,
                    },
                ))
            })
        })
        .collect::<Option<_>>()
}

fn eval_root(map: &HashMap<&str, Job>) -> Result<i64, String> {
    let root = map.get("root").ok_or_else(|| "No root".to_string())?;

    eval(root, map)
}

fn part1(input: &str) -> Result<String, String> {
    let map = parse(input).ok_or_else(|| "Invalid input".to_string())?;

    let root = map.get("root").ok_or_else(|| "No root".to_string())?;

    eval(root, &map).map(|number| number.to_string())
}

fn find_zero(mut lower: i64, mut upper: i64, map: &mut HashMap<&str, Job>) -> Result<i64, String> {
    map.insert("humn", Job::Number(lower));

    let lower_sign = eval_root(map)?.signum();

    if lower_sign == 0 {
        return Ok(lower);
    }

    map.insert("humn", Job::Number(upper));

    let upper_sign = eval_root(map)?.signum();

    if upper_sign == 0 {
        return Ok(upper);
    }

    while lower != upper {
        let mid = (lower + upper) / 2;
        map.insert("humn", Job::Number(mid));

        let mid_sign = eval_root(map)?.signum();

        if mid_sign == 0 {
            return Ok(mid);
        }

        if mid_sign == lower_sign {
            lower = mid;
        } else {
            upper = mid
        }
    }

    Err("Not found".to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut map = parse(input).ok_or_else(|| "Invalid input".to_string())?;

    let root = map.get_mut("root").ok_or_else(|| "No root".to_string())?;

    if let Job::Expr(_, op, _) = root {
        *op = Op::Sub;
    }

    for i in 0..60 {
        let upper = 1 << i;
        let lower = -upper;

        map.insert("humn", Job::Number(lower));

        let lower_sign = eval_root(&map)?.signum();

        map.insert("humn", Job::Number(upper));

        let upper_sign = eval_root(&map)?.signum();

        if lower_sign != upper_sign {
            return find_zero(lower, upper, &mut map).map(|number| number.to_string());
        }
    }

    Err("Not found".to_string())
}

advent_of_code::aoc_main!(21);
