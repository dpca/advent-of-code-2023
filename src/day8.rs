use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// Common

#[derive(Debug)]
struct MapNode {
    left: String,
    right: String,
}

fn file_reader() -> io::BufReader<File> {
    let file = File::open("./inputs/day8.txt").expect("File not found");
    return io::BufReader::new(file);
}

fn parse_node(line: String) -> (String, MapNode) {
    return (
        (&line[0..3]).to_string(),
        MapNode {
            left: (&line[7..10]).to_string(),
            right: (&line[12..15]).to_string(),
        },
    );
}

fn parse_input() -> (String, HashMap<String, MapNode>) {
    let mut reader = file_reader();
    let mut instructions = String::new();
    reader.read_line(&mut instructions).expect("No data found");

    let mut buf = String::new();
    reader.read_line(&mut buf).expect("Empty line expected");

    let mut desert_map = HashMap::new();
    for line in reader.lines() {
        let (position, node) = parse_node(line.expect("lines failed"));
        desert_map.insert(position, node);
    }

    return (instructions.trim().to_string(), desert_map);
}

fn steps_for_input(
    starting_node: &str,
    ending_node_fn: &dyn Fn(&str) -> bool,
    instructions: Vec<char>,
    desert_map: &HashMap<String, MapNode>,
) -> u64 {
    let num_instructions: usize = instructions.len();
    let mut steps: usize = 0;
    let mut current_node = starting_node;

    loop {
        if ending_node_fn(current_node) {
            break;
        }

        let instruction = instructions[steps % num_instructions];

        if instruction == 'L' {
            current_node = &desert_map[current_node].left;
        } else {
            current_node = &desert_map[current_node].right;
        }

        steps += 1;
    }

    return steps as u64;
}

// Part 1

fn part1() -> u64 {
    let (instructions, desert_map) = parse_input();

    fn stop_fn(node: &str) -> bool {
        return node == "ZZZ";
    }

    return steps_for_input("AAA", &stop_fn, instructions.chars().collect(), &desert_map);
}

// Part 2

fn least_common_factor(nums: Vec<u64>) -> u64 {
    if nums.len() > 1 {
        let mut num1 = nums[0];
        let mut num2 = nums[1];

        while num1 != num2 {
            if num1 < num2 {
                num1 += nums[0];
            } else {
                num2 += nums[1];
            }
        }

        return least_common_factor([&[num1], &nums[2..]].concat());
    }

    return nums[0];
}

fn part2() -> u64 {
    let (instructions, desert_map) = parse_input();

    fn stop_fn(node: &str) -> bool {
        return node.ends_with("Z");
    }

    let starting_nodes: Vec<String> = desert_map
        .keys()
        .map(|n| n.clone())
        .filter(|n| n.ends_with("A"))
        .collect();
    let all_nodes_least_paths: Vec<u64> = starting_nodes
        .iter()
        .map(|n| steps_for_input(&n, &stop_fn, instructions.chars().collect(), &desert_map))
        .collect();

    return least_common_factor(all_nodes_least_paths);
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
        assert_eq!(part1(), 20221);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 14616363770447);
    }
}
