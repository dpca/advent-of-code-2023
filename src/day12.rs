use itertools::Itertools;
use std::fs;

// Common

#[derive(Debug, PartialEq, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    springs: Vec<SpringState>,
    nums: Vec<u32>,
}

fn parse_spring_state(input: char) -> SpringState {
    return match input {
        '.' => SpringState::Operational,
        '#' => SpringState::Damaged,
        '?' => SpringState::Unknown,
        _ => panic!("Unknown spring state {input}"),
    };
}

#[allow(dead_code)]
fn print_row(input: &Vec<SpringState>) {
    let mut out: String = "".to_string();
    for loc in input {
        match loc {
            SpringState::Operational => out.push('.'),
            SpringState::Damaged => out.push('#'),
            SpringState::Unknown => out.push('?'),
        }
    }
    out.push('\n');
    print!("{}", out);
}

fn parse_line(line: &str) -> Row {
    let (spring_strings, num_strings) = line.split_once(" ").unwrap();
    let springs: Vec<SpringState> = spring_strings.chars().map(parse_spring_state).collect();
    let nums: Vec<u32> = num_strings.split(",").map(|n| n.parse().unwrap()).collect();
    return Row { springs, nums };
}

fn parse_input() -> Vec<Row> {
    let contents = fs::read_to_string("./inputs/day12.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|line| line.len() > 0)
        .map(parse_line)
        .collect();
}

fn num_arrangements(row: &Row) -> u32 {
    let num_groups = row.nums.len() as u32;
    let fill_required = row.nums.iter().sum::<u32>() + num_groups - 1;
    let num_empty = row.springs.len() as u32 - fill_required;
    let options = (0..(num_groups + num_empty)).combinations(num_groups as usize);

    let mut arrangements = 0;
    for option in options {
        let mut fill = vec![SpringState::Operational; row.springs.len()];
        let mut skip = 0;
        for (idx, group) in option.iter().enumerate() {
            for i in 0..row.nums[idx] {
                fill[(*group + i + skip) as usize] = SpringState::Damaged;
            }
            skip += row.nums[idx];
        }
        if (0..fill.len()).all(|i| {
            (fill[i] == SpringState::Operational
                && (row.springs[i] == SpringState::Operational
                    || row.springs[i] == SpringState::Unknown))
                || (fill[i] == SpringState::Damaged
                    && (row.springs[i] == SpringState::Damaged
                        || row.springs[i] == SpringState::Unknown))
        }) {
            arrangements += 1;
        }
    }
    return arrangements;
}

// Part 1

fn part1() -> u32 {
    let rows = parse_input();
    return rows.iter().map(num_arrangements).sum();
}

// Part 2

fn unfold_row(row: &Row) -> Row {
    let mut new_row: Row = Row {
        springs: Vec::new(),
        nums: Vec::new(),
    };
    for i in 0..5 {
        for num in &row.nums {
            new_row.nums.push(*num);
        }
        for spring in &row.springs {
            new_row.springs.push(*spring);
        }
        if i != 4 {
            new_row.springs.push(SpringState::Unknown);
        }
    }
    return new_row;
}

fn part2() -> u32 {
    return 1;
    let rows = parse_input();
    return rows
        .iter()
        .map(|row| num_arrangements(&unfold_row(row)))
        .sum();
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
        assert_eq!(part1(), 7191);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1);
    }
}
