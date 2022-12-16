pub fn run(
    day: i32,
    part1: impl Fn(&str) -> Result<String, String>,
    part2: impl Fn(&str) -> Result<String, String>,
) {
    let input_path = format!("inputs/day{day:02}.txt");
    let input = std::fs::read_to_string(&input_path);

    let result = match input {
        Ok(input) => format!(
            "Part 1: {}\nPart 2: {}\n",
            part1(&input).unwrap_or_else(|err| format!("Error - {err}")),
            part2(&input).unwrap_or_else(|err| format!("Error - {err}"))
        ),
        Err(err) => format!("Error reading input file \"{input_path}\": {}\n", err),
    };
    println!("Advent of Code Day {day}\n{result}");
}

#[macro_export]
macro_rules! aoc_main {
    ($day:expr) => {
        fn main() {
            $crate::run($day, part1, part2);
        }
    };
}
