use std::collections::HashMap;
use std::fs;

// Common

fn parse_input() -> Vec<String> {
    let mut contents = fs::read_to_string("./inputs/day15.txt").expect("No file found");
    if contents.ends_with('\n') {
        contents.pop();
    }
    return contents.split(",").map(|s| s.trim().to_string()).collect();
}

fn string_code(input: &String) -> u32 {
    let mut code: u32 = 0;
    for char in input.chars() {
        code += char as u32;
        code *= 17;
        code = code % 256;
    }
    return code;
}

// Part 1

fn part1() -> u32 {
    return parse_input().iter().map(string_code).sum();
}

// Part 2

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn initialize_boxes() -> HashMap<u32, Vec<Lens>> {
    let mut boxes = HashMap::new();
    for i in 0..256 {
        boxes.insert(i, Vec::new());
    }
    return boxes;
}

fn focusing_power(box_num: &u32, lenses: &Vec<Lens>) -> u32 {
    return lenses
        .iter()
        .enumerate()
        .map(|(idx, lens)| (box_num + 1) * (idx as u32 + 1) * lens.focal_length as u32)
        .sum();
}

fn part2() -> u32 {
    let mut boxes = initialize_boxes();
    for instruction in parse_input() {
        match instruction.chars().find(|c| c == &'-' || c == &'=') {
            Some('-') => {
                let mut label = instruction;
                label.pop();
                boxes.entry(string_code(&label)).and_modify(|lenses| {
                    lenses.retain(|l| l.label != label);
                });
            }
            Some('=') => {
                let (label_str, focal_length_str) = instruction.split_once("=").unwrap();
                let label = label_str.to_string();
                let focal_length = focal_length_str.parse().unwrap();
                let lens = Lens {
                    label,
                    focal_length,
                };
                boxes.entry(string_code(&lens.label)).and_modify(|lenses| {
                    let found_idx = lenses.iter().position(|l| l.label == label_str);
                    match found_idx {
                        Some(idx) => lenses[idx] = lens,
                        None => lenses.push(lens),
                    }
                });
            }
            _ => panic!("Unknown operation in {instruction}"),
        }
    }

    return boxes.iter().map(|(k, v)| focusing_power(k, v)).sum();
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
        assert_eq!(part1(), 505427);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 243747);
    }
}
