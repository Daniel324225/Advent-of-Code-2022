#[derive(PartialEq, Eq, Ord)]
enum Data {
    Int(i32),
    List(Vec<Data>),
}

impl std::cmp::PartialOrd for Data {
    fn partial_cmp(&self, rhs: &Data) -> Option<std::cmp::Ordering> {
        use Data::*;
        match (self, rhs) {
            (Int(a), Int(b)) => a.partial_cmp(b),
            (List(a), List(b)) => a.partial_cmp(b),
            (Int(a), List(b)) => vec![Int(*a)].partial_cmp(b),
            (a, b) => b.partial_cmp(a).map(|ord| ord.reverse()),
        }
    }
}

fn parse(input: &str) -> Result<Data, String> {
    let input = input.trim();

    let mut open_count = 0;
    let test = |c: char| {
        match c {
            '[' => open_count += 1,
            ']' => open_count -= 1,
            ',' if open_count == 0 => {
                return true;
            }
            _ => {}
        }
        false
    };
    match input.chars().next() {
        Some('[') => {
            let input = input
                .strip_prefix('[')
                .and_then(|input| input.strip_suffix(']'))
                .ok_or_else(|| "Invalid input".to_string())?;
            if input.is_empty() {
                Ok(Data::List(Vec::new()))
            } else {
                Ok(Data::List(
                    input
                        .split(test)
                        .map(parse)
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            }
        }
        _ => Ok(Data::Int(
            input.parse().map_err(|_| "Invalid input".to_string())?,
        )),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let mut packets = input.lines().filter(|line| !line.is_empty());

    (1..)
        .zip(
            std::iter::from_fn(|| Some((packets.next()?, packets.next()?)))
                .map(|(a, b)| Ok::<(Data, Data), String>((parse(a)?, parse(b)?))),
        )
        .try_fold(0, |sum, (index, pair)| {
            pair.map(|(a, b)| if a < b { sum + index } else { sum })
        })
        .map(|sum| sum.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut packets: Vec<Data> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse)
        .collect::<Result<_, _>>()?;

    packets.sort();

    let a = Data::List(vec![Data::List(vec![Data::Int(2)])]);
    let b = Data::List(vec![Data::List(vec![Data::Int(6)])]);
    let a = packets.partition_point(|p| *p < a);
    let b = packets.partition_point(|p| *p < b);

    Ok(((a + 1) * (b + 2)).to_string())
}

advent_of_code::aoc_main!(13);
