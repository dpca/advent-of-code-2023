use std::fs;

// Common

#[derive(Debug, PartialEq)]
enum Ground {
    Ash,
    Rock,
}

type Puzzle = Vec<Vec<Ground>>;

fn parse_ground(ground: char) -> Ground {
    return match ground {
        '.' => Ground::Ash,
        '#' => Ground::Rock,
        _ => panic!("Unknown ground type {ground}"),
    };
}

fn parse_puzzle(input: &str) -> Puzzle {
    return input
        .lines()
        .map(|line| line.chars().map(parse_ground).collect())
        .collect();
}

fn parse_input() -> Vec<Puzzle> {
    let contents = fs::read_to_string("./inputs/day13.txt").expect("No file found");
    let puzzles = contents.split("\n\n");
    return puzzles.map(parse_puzzle).collect();
}

fn column_reflects(puzzle: &Puzzle, col: usize, diffs_allowed: u32) -> bool {
    let mut num_diffs = 0;
    for i in 0..puzzle[0].len() {
        if col < i || col + 1 + i >= puzzle[0].len() {
            return num_diffs == diffs_allowed;
        }
        for row in 0..puzzle.len() {
            if puzzle[row][col - i] != puzzle[row][col + i + 1] {
                num_diffs += 1;
                if num_diffs > diffs_allowed {
                    return false;
                }
            }
        }
    }
    return num_diffs == diffs_allowed;
}

fn row_reflects(puzzle: &Puzzle, row: usize, diffs_allowed: u32) -> bool {
    let mut num_diffs = 0;
    for i in 0..puzzle.len() {
        if row < i || row + 1 + i >= puzzle.len() {
            return num_diffs == diffs_allowed;
        }
        for col in 0..puzzle[0].len() {
            if puzzle[row - i][col] != puzzle[row + i + 1][col] {
                num_diffs += 1;
                if num_diffs > diffs_allowed {
                    return false;
                }
            }
        }
    }
    return num_diffs == diffs_allowed;
}

fn find_mirror(puzzle: &Puzzle, diffs_allowed: u32) -> (bool, u32) {
    for col in 0..(puzzle[0].len() - 1) {
        if column_reflects(puzzle, col, diffs_allowed) {
            return (false, col as u32 + 1);
        }
    }
    for row in 0..(puzzle.len() - 1) {
        if row_reflects(puzzle, row, diffs_allowed) {
            return (true, row as u32 + 1);
        }
    }
    panic!("No mirror found for puzzle!");
}

fn mirror_num(puzzle: &Puzzle, diffs_allowed: u32) -> u32 {
    let (row_match, num) = find_mirror(&puzzle, diffs_allowed);
    if row_match {
        return num * 100;
    } else {
        return num;
    }
}

// Part 1

fn part1() -> u32 {
    let puzzles = parse_input();
    return puzzles.iter().map(|p| mirror_num(&p, 0)).sum();
}

// Part 2

fn part2() -> u32 {
    let puzzles = parse_input();
    return puzzles.iter().map(|p| mirror_num(&p, 1)).sum();
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
        assert_eq!(part1(), 43614);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 36771);
    }
}
