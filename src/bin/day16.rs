fn parse(input: &str) -> Option<Vec<(i32, Vec<i32>)>> {
    let regex =
        regex::Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)")
            .unwrap();

    let mut nodes = std::collections::HashMap::new();
    nodes.insert("AA".to_string(), 0usize);

    let mut graph = Vec::<(i32, Vec<usize>)>::new();
    graph.push((0, Vec::new()));

    let mut get_node = |key: &str| {
        let len = nodes.len();
        *nodes.entry(key.to_string()).or_insert(len)
    };

    for line in input.lines() {
        let captures = regex.captures(line)?;
        let node = get_node(captures.get(1).unwrap().as_str());
        if node == graph.len() {
            graph.push((
                captures.get(2).unwrap().as_str().parse().unwrap(),
                Vec::new(),
            ));
        } else {
            let node = graph.get_mut(node).unwrap();
            node.0 = captures.get(2).unwrap().as_str().parse().unwrap();
        };

        for neighbor in captures
            .get(3)
            .unwrap()
            .as_str()
            .split(',')
            .map(|str| get_node(str.trim()))
        {
            if neighbor == graph.len() {
                graph.push((0, Vec::new()));
            }
            graph[node].1.push(neighbor);
        }
    }

    let mut reduced_nodes = std::collections::HashMap::new();
    reduced_nodes.insert(0, 0);
    for (node, (value, _)) in graph.iter().enumerate() {
        if *value != 0 {
            reduced_nodes.insert(node, reduced_nodes.len());
        }
    }

    let mut reduced_graph = vec![(0, vec![0; reduced_nodes.len()]); reduced_nodes.len()];

    for (node, (value, _)) in graph.iter().enumerate() {
        let reduced_node;

        if let Some(v) = reduced_nodes.get(&node) {
            reduced_node = *v;
        } else {
            continue;
        }

        reduced_graph[reduced_node].0 = *value;

        let mut visited = vec![false; nodes.len()];
        let mut queue = std::collections::VecDeque::new();

        visited[node] = true;
        queue.push_back((node, 1));

        while let Some((node, distance)) = queue.pop_front() {
            if let Some(&next) = reduced_nodes.get(&node) {
                reduced_graph[reduced_node].1[next] = distance;
            }

            for &next in graph[node].1.iter() {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back((next, distance + 1));
                }
            }
        }
    }

    Some(reduced_graph)
}

fn dfs<const MAX: i32>(graph: &Vec<(i32, Vec<i32>)>) -> i32 {
    let mut visited = vec![false; graph.len()];
    let max_flow = graph.iter().map(|(flow, _)| *flow).sum();

    let mut max_score = 0;

    fn implementation<const MAX: i32>(
        graph: &Vec<(i32, Vec<i32>)>,
        node: usize,
        distance: i32,
        mut current_score: i32,
        visited: &mut [bool],
        max_score: &mut i32,
        max_flow: i32,
    ) {
        let (flow, costs) = graph.get(node).unwrap();
        let max_flow = max_flow - flow;

        current_score += flow * (MAX - distance);
        *max_score = (*max_score).max(current_score);

        visited[node] = true;

        for (next, cost) in costs.iter().copied().enumerate() {
            let distance = distance + cost;
            if !visited[next]
                && distance <= MAX
                && max_flow * (MAX - distance) + current_score > *max_score
            {
                implementation::<MAX>(
                    graph,
                    next,
                    distance,
                    current_score,
                    visited,
                    max_score,
                    max_flow,
                );
            }
        }

        visited[node] = false;
    }

    implementation::<MAX>(graph, 0, 0, 0, &mut visited, &mut max_score, max_flow);
    max_score
}

fn dfs2<const MAX: i32>(graph: &Vec<(i32, Vec<i32>)>) -> i32 {
    let mut visited = vec![false; graph.len()];
    let max_flow = graph.iter().map(|(flow, _)| *flow).sum();

    let mut max_score = 0;

    fn implementation<const MAX: i32>(
        graph: &Vec<(i32, Vec<i32>)>,
        nodes: (usize, usize),
        distance: (i32, i32),
        mut current_score: i32,
        visited: &mut [bool],
        max_score: &mut i32,
        max_flow: i32,
    ) {
        let (flow0, costs0) = graph.get(nodes.0).unwrap();
        let (flow1, costs1) = graph.get(nodes.1).unwrap();
        let max_flow = max_flow - flow0 - flow1;

        current_score += flow0 * (MAX - distance.0);
        current_score += flow1 * (MAX - distance.1);

        *max_score = (*max_score).max(current_score);

        visited[nodes.0] = true;
        visited[nodes.1] = true;

        for (next0, cost0) in costs0.iter().copied().enumerate() {
            let distance0 = distance.0 + cost0;
            if visited[next0] || distance0 > MAX {
                continue;
            }
            for (next1, cost1) in costs1.iter().copied().enumerate() {
                let distance1 = distance.1 + cost1;
                if next0 == next1 || visited[next1] || distance1 > MAX {
                    continue;
                }
                if max_flow * (MAX - distance0.min(distance1)) + current_score > *max_score {
                    implementation::<MAX>(
                        graph,
                        (next0, next1),
                        (distance0, distance1),
                        current_score,
                        visited,
                        max_score,
                        max_flow,
                    );
                }
            }
        }

        visited[nodes.0] = false;
        visited[nodes.1] = false;
    }

    implementation::<MAX>(
        graph,
        (0, 0),
        (0, 0),
        0,
        &mut visited,
        &mut max_score,
        max_flow,
    );
    max_score
}

fn part1(input: &str) -> Result<String, String> {
    let graph = parse(input).ok_or_else(|| "invalid input".to_string())?;

    Ok(dfs::<30>(&graph).to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut graph = parse(input).ok_or_else(|| "invalid input".to_string())?;
    for node in graph.iter_mut() {
        node.1.push(0);
    }
    graph.push((0, vec![26; graph.len() + 1]));

    Ok(dfs2::<26>(&graph).to_string())
}

advent_of_code::aoc_main!(16);
