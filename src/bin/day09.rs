#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn part1(input: &str) -> Result<String, String> {
    let mut head = Vec2 { x: 0, y: 0 };
    let mut tail = head;

    let mut visited = std::collections::HashSet::<Vec2>::new();
    visited.insert(tail);

    for r#move in input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let dir = parts.next()?;
        let count: i32 = parts.next()?.parse().ok()?;

        let dir = match dir {
            "R" => Vec2::from(1, 0),
            "L" => Vec2::from(-1, 0),
            "U" => Vec2::from(0, 1),
            "D" => Vec2::from(0, -1),
            _ => return None,
        };

        Some((dir, count))
    }) {
        let (dir, count) = r#move.ok_or_else(|| "Invalid input".to_string())?;
        for _ in 0..count {
            head += dir;
            let mut diff = head - tail;
            if diff.x.abs().max(diff.y.abs()) >= 2 {
                diff.x = diff.x.clamp(-1, 1);
                diff.y = diff.y.clamp(-1, 1);
                tail += diff;
                visited.insert(tail);
            }
        }
    }

    Ok(visited.len().to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let mut knots = [Vec2 { x: 0, y: 0 }; 10];

    let mut visited = std::collections::HashSet::<Vec2>::new();
    visited.insert(*knots.first().unwrap());

    for r#move in input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let dir = parts.next()?;
        let count: i32 = parts.next()?.parse().ok()?;

        let dir = match dir {
            "R" => Vec2::from(1, 0),
            "L" => Vec2::from(-1, 0),
            "U" => Vec2::from(0, 1),
            "D" => Vec2::from(0, -1),
            _ => return None,
        };

        Some((dir, count))
    }) {
        let (dir, count) = r#move.ok_or_else(|| "Invalid input".to_string())?;
        for _ in 0..count {
            *knots.first_mut().unwrap() += dir;
            for head in 0..knots.len() - 1 {
                let tail = head + 1;
                let mut diff = knots[head] - knots[tail];
                if diff.x.abs().max(diff.y.abs()) >= 2 {
                    diff.x = diff.x.clamp(-1, 1);
                    diff.y = diff.y.clamp(-1, 1);
                    knots[tail] += diff;
                }
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    Ok(visited.len().to_string())
}

advent_of_code::aoc_main!(9);
