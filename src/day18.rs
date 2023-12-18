use std::collections::HashMap;
use std::fs;

// Common

#[derive(PartialEq)]
enum Node {
    Trench,
    Ground,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: u64,
}

fn parse_input(parse_line: &dyn Fn(&str) -> Instruction) -> Vec<Instruction> {
    let contents = fs::read_to_string("./inputs/day18.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines.filter(|l| l.len() > 0).map(parse_line).collect();
}

#[allow(dead_code)]
fn print_map(dig_map: &Vec<Vec<Node>>) {
    for row in dig_map {
        for node in row {
            match node {
                Node::Trench => print!("#"),
                Node::Ground => print!("."),
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn flood_map(map: &mut Vec<Vec<Node>>) {
    let row = map.len() / 2;
    let mut col = 0;

    let mut found_boundary = false;
    while !found_boundary {
        col += 1;
        if map[row][col] == Node::Trench {
            found_boundary = true;
        }
    }

    let mut crossed_boundary = false;
    while !crossed_boundary {
        col += 1;
        if map[row][col] == Node::Ground {
            crossed_boundary = true;
        }
    }

    let mut frontier = Vec::new();
    frontier.push((row, col));

    while let Some((x, y)) = frontier.pop() {
        map[x][y] = Node::Trench;
        if x > 0 && map[x - 1][y] == Node::Ground {
            frontier.push((x - 1, y));
        }
        if x < map.len() && map[x + 1][y] == Node::Ground {
            frontier.push((x + 1, y));
        }
        if col > 0 && map[x][y - 1] == Node::Ground {
            frontier.push((x, y - 1));
        }
        if col < map[0].len() && map[x][y + 1] == Node::Ground {
            frontier.push((x, y + 1));
        }
    }
}

fn get_lava(instructions: &Vec<Instruction>) -> u64 {
    let mut row_min = 0;
    let mut row_max = 0;
    let mut col_min = 0;
    let mut col_max = 0;
    let mut position: (i64, i64) = (0, 0);

    let mut dig_row_ranges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    let mut dig_col_ranges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                dig_col_ranges
                    .entry(position.1)
                    .or_insert_with(|| Vec::new())
                    .push((position.0 - instruction.length as i64, position.0));
                position = (position.0 - instruction.length as i64, position.1);
            }
            Direction::Down => {
                dig_col_ranges
                    .entry(position.1)
                    .or_insert_with(|| Vec::new())
                    .push((position.0, position.0 + instruction.length as i64));
                position = (position.0 + instruction.length as i64, position.1);
            }
            Direction::Right => {
                dig_row_ranges
                    .entry(position.0)
                    .or_insert_with(|| Vec::new())
                    .push((position.1, position.1 + instruction.length as i64));
                position = (position.0, position.1 + instruction.length as i64);
            }
            Direction::Left => {
                dig_row_ranges
                    .entry(position.0)
                    .or_insert_with(|| Vec::new())
                    .push((position.1 - instruction.length as i64, position.1));
                position = (position.0, position.1 - instruction.length as i64);
            }
        }

        if position.0 < row_min {
            row_min = position.0;
        } else if position.0 > row_max {
            row_max = position.0;
        }

        if position.1 < col_min {
            col_min = position.1;
        } else if position.1 > col_max {
            col_max = position.1;
        }
    }

    let mut full_map = Vec::new();
    for row in row_min..row_max + 1 {
        let mut map_row = Vec::new();
        for col in col_min..col_max + 1 {
            if dig_row_ranges.get(&row).is_some_and(|ranges| {
                ranges
                    .iter()
                    .any(|(start, end)| start <= &col && &col <= end)
            }) || dig_col_ranges.get(&col).is_some_and(|ranges| {
                ranges
                    .iter()
                    .any(|(start, end)| start <= &row && &row <= end)
            }) {
                map_row.push(Node::Trench);
            } else {
                map_row.push(Node::Ground);
            }
        }
        full_map.push(map_row);
    }

    flood_map(&mut full_map);

    let mut count = 0;
    for row in full_map {
        for node in row {
            if node == Node::Trench {
                count += 1;
            }
        }
    }
    return count;
}

// Part 1

fn parse_direction_1(input: &str) -> Direction {
    match input {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction {input}"),
    }
}

fn parse_line_1(input: &str) -> Instruction {
    let splits: Vec<&str> = input.split_whitespace().collect();

    Instruction {
        direction: parse_direction_1(splits[0]),
        length: splits[1].parse().unwrap(),
    }
}

fn part1() -> u64 {
    let instructions = parse_input(&parse_line_1);
    get_lava(&instructions)
}

// Part 2

fn parse_direction_2(input: &str) -> Direction {
    match input {
        "3" => Direction::Up,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "0" => Direction::Right,
        _ => panic!("Unknown direction {input}"),
    }
}

fn parse_line_2(input: &str) -> Instruction {
    let splits: Vec<&str> = input.split_whitespace().collect();
    let hex_str = splits[2][1..splits[2].len() - 1].to_string();

    Instruction {
        direction: parse_direction_2(&hex_str[hex_str.len() - 1..hex_str.len()]),
        length: u64::from_str_radix(&hex_str[1..hex_str.len() - 1], 16).unwrap(),
    }
}

fn part2() -> u64 {
    return 1;
    let instructions = parse_input(&parse_line_2);

    get_lava(&instructions)
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
        assert_eq!(part1(), 49061);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1);
    }
}
