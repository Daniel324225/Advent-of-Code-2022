use std::collections::HashMap;
const ROBOTS: usize = 4;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    robots: [i32; ROBOTS],
    inventory: [i32; ROBOTS],
    time: i32,
}

fn implementation(
    state: State,
    costs: &[[i32; 2]; ROBOTS],
    max_needed: &[i32; 4],
    cache: &mut HashMap<State, i32>,
) -> i32 {
    if state.robots[2] == max_needed[2] {
        let mut max = *state.inventory.last().unwrap();
        max += state.time * state.robots.last().unwrap();
        max += (1 + state.time - 1) * (state.time - 1) / 2;

        cache.insert(state, max);
        return max;
    }

    let time_needed = |needed: [i32; 2], production: [i32; 2]| {
        if production[0] == 0 || production[1] == 0 {
            i32::MAX - 1
        } else {
            needed
                .iter()
                .zip(&production)
                .map(|(&needed, &production)| {
                    needed / production + if needed % production == 0 { 0 } else { 1 }
                })
                .max()
                .unwrap()
        }
    };

    let mut max = state.time * state.robots.last().unwrap() + state.inventory.last().unwrap();
    for robot in 0..ROBOTS {
        if max_needed[robot] == state.robots[robot] {
            continue;
        }
        let mats_needed = [
            (costs[robot][0] - state.inventory[0]).max(0),
            if robot == 0 {
                0
            } else {
                (costs[robot][1] - state.inventory[robot - 1]).max(0)
            },
        ];

        let production = [
            state.robots[0],
            if robot == 0 {
                1
            } else {
                state.robots[robot - 1]
            },
        ];

        let time_needed = time_needed(mats_needed, production) + 1;
        if time_needed <= state.time {
            let mut state = state;
            state.time -= time_needed;

            for (mat, production) in state.inventory.iter_mut().zip(state.robots) {
                *mat += production * time_needed;
            }

            state.inventory[0] -= costs[robot][0];
            if robot != 0 {
                state.inventory[robot - 1] -= costs[robot][1];
            }

            state.robots[robot] += 1;

            let diff = cache.get(&state).copied();
            let diff = diff.unwrap_or_else(|| implementation(state, costs, max_needed, cache));
            max = max.max(diff);
        }
    }
    cache.insert(state, max);
    max
}

fn parse(input: &str) -> Result<(i32, [[i32; 2]; 4], [i32; 4]), String> {
    let regex = regex::Regex::new(r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap();

    let captures = regex.captures(input).ok_or_else(|| "Invalid input")?;

    let get = |id| captures.get(id).unwrap().as_str().parse().unwrap();

    let costs = [[get(2), 0], [get(3), 0], [get(4), get(5)], [get(6), get(7)]];
    let max_needed = [
        costs.iter().map(|[a, _]| *a).max().unwrap(),
        costs[2][1],
        costs[3][1],
        i32::MAX,
    ];

    return Ok((get(1), costs, max_needed));
}

fn part1(input: &str) -> Result<String, String> {
    let mut i = 0;
    input
        .lines()
        .inspect(|_| {
            println!("{}", i);
            i += 1;
        })
        .map(parse)
        .try_fold(0, |sum, parsed| {
            parsed.map(|(id, costs, max_needed)| {
                sum + id as i64
                    * implementation(
                        State {
                            robots: [1, 0, 0, 0],
                            inventory: [0; 4],
                            time: 24,
                        },
                        &costs,
                        &max_needed,
                        &mut HashMap::new(),
                    ) as i64
            })
        })
        .map(|sum| sum.to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut i = 0;
    input
        .lines()
        .take(3)
        .inspect(|_| {
            println!("{}", i);
            i += 1;
        })
        .map(parse)
        .try_fold(1, |score, parsed| {
            parsed.map(|(_, costs, max_needed)| {
                score
                    * implementation(
                        State {
                            robots: [1, 0, 0, 0],
                            inventory: [0; 4],
                            time: 32,
                        },
                        &costs,
                        &max_needed,
                        &mut HashMap::new(),
                    ) as i64
            })
        })
        .map(|sum| sum.to_string())
}

advent_of_code::aoc_main!(19);
