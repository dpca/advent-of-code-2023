use std::fs;

// Common

fn parse_input() -> Vec<Vec<i32>> {
    let contents = fs::read_to_string("./inputs/day9.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(" ")
                .map(|i| i.parse().expect("Couldn't parse number"))
                .collect()
        })
        .collect();
}

fn step_differences(inputs: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::new();
    for i in 1..inputs.len() {
        diffs.push(inputs[i] - inputs[i - 1]);
    }
    return diffs;
}

// Part 1

fn next_history_value(inputs: &Vec<i32>) -> i32 {
    if inputs.iter().all(|i| *i == 0) {
        return 0;
    }
    return inputs.last().unwrap() + next_history_value(&step_differences(inputs));
}

fn part1() -> i32 {
    return parse_input().iter().map(next_history_value).sum();
}

// Part 2

fn previous_history_value(inputs: &Vec<i32>) -> i32 {
    if inputs.iter().all(|i| *i == 0) {
        return 0;
    }
    return inputs.first().unwrap() - previous_history_value(&step_differences(inputs));
}

fn part2() -> i32 {
    return parse_input().iter().map(previous_history_value).sum();
}

// Main

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1916822650);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 966);
    }
}
