use std::collections::HashMap;
use std::fs;

// Common

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq)]
enum Operation {
    GT,
    LT,
}

#[derive(Debug)]
struct Rule {
    category: Category,
    operation: Operation,
    value: u32,
    destination: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: String,
}

#[derive(Debug)]
struct Part {
    rating_x: u32,
    rating_m: u32,
    rating_a: u32,
    rating_s: u32,
}

fn parse_rule(input: &str) -> Rule {
    let mut input_chars = input.chars();

    let category = match input_chars.next() {
        Some('x') => Category::X,
        Some('m') => Category::M,
        Some('a') => Category::A,
        Some('s') => Category::S,
        _ => panic!("Unknown category"),
    };

    let operation = match input_chars.next() {
        Some('<') => Operation::LT,
        Some('>') => Operation::GT,
        _ => panic!("Unknown rule"),
    };

    let input_rest = input_chars.collect::<String>();
    let (value, destination) = input_rest.split_once(":").unwrap();

    Rule {
        category,
        operation,
        value: value.parse().unwrap(),
        destination: destination.to_string(),
    }
}

fn parse_workflow(input: &str) -> Workflow {
    let (name, rules_str) = input.split_once("{").unwrap();
    let mut rules: Vec<&str> = rules_str[0..rules_str.len() - 1].split(",").collect();
    let default = rules.pop().unwrap();

    Workflow {
        name: name.to_string(),
        rules: rules.iter().map(|r| parse_rule(*r)).collect(),
        default: default.to_string(),
    }
}

fn get_rating(input: &str) -> u32 {
    let mut input_chars = input.chars();
    input_chars.next();
    input_chars.next();
    let input_rest = input_chars.collect::<String>();
    input_rest.parse().unwrap()
}

fn parse_part(input: &str) -> Part {
    let ratings: Vec<&str> = input[1..input.len() - 1].split(",").collect();
    let rating_x = get_rating(ratings[0]);
    let rating_m = get_rating(ratings[1]);
    let rating_a = get_rating(ratings[2]);
    let rating_s = get_rating(ratings[3]);

    Part {
        rating_x,
        rating_m,
        rating_a,
        rating_s,
    }
}

fn parse_input() -> (HashMap<String, Workflow>, Vec<Part>) {
    let contents = fs::read_to_string("./inputs/day19.txt").expect("File not found");
    let (workflows_str, ratings_str) = contents.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();

    for workflow in workflows_str.split("\n").map(parse_workflow) {
        workflows.insert(workflow.name.clone(), workflow);
    }

    (
        workflows,
        ratings_str
            .split("\n")
            .filter(|s| s.len() > 0)
            .map(parse_part)
            .collect(),
    )
}

// Part 1

fn run_workflow(part: &Part, workflow: &Workflow) -> String {
    for rule in &workflow.rules {
        if rule.operation == Operation::LT {
            match rule.category {
                Category::X => {
                    if part.rating_x < rule.value {
                        return rule.destination.to_string();
                    }
                }
                Category::M => {
                    if part.rating_m < rule.value {
                        return rule.destination.to_string();
                    }
                }
                Category::A => {
                    if part.rating_a < rule.value {
                        return rule.destination.to_string();
                    }
                }
                Category::S => {
                    if part.rating_s < rule.value {
                        return rule.destination.to_string();
                    }
                }
            }
        } else {
            match rule.category {
                Category::X => {
                    if part.rating_x > rule.value {
                        return rule.destination.to_string();
                    }
                }
                Category::M => {
                    if part.rating_m > rule.value {
                        return rule.destination.to_string();
                    }
                }
                Category::A => {
                    if part.rating_a > rule.value {
                        return rule.destination.to_string();
                    }
                }
                Category::S => {
                    if part.rating_s > rule.value {
                        return rule.destination.to_string();
                    }
                }
            }
        }
    }
    return workflow.default.to_string();
}

fn part_rating(part: &Part) -> u32 {
    return part.rating_x + part.rating_m + part.rating_a + part.rating_s;
}

fn part1() -> u32 {
    let (workflows, parts) = parse_input();

    let mut accepted_parts: Vec<Part> = Vec::new();
    for part in parts {
        let mut workflow = "in".to_string();
        while workflow != "A" && workflow != "R" {
            workflow = run_workflow(&part, &workflows[&workflow]);
        }
        if workflow == "A" {
            accepted_parts.push(part);
        }
    }

    return accepted_parts.iter().map(part_rating).sum();
}

// Part 2

fn part2() -> u32 {
    return 1;
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
        assert_eq!(part1(), 373302);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1);
    }
}
