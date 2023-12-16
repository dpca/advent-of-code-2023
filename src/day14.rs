use std::collections::HashMap;
use std::fs;

// Common

#[derive(Debug, PartialEq, Clone, Copy)]
enum Position {
    RoundedRock,
    CubeRock,
    Empty,
}

type Mirror = Vec<Vec<Position>>;

fn parse_position(position: char) -> Position {
    return match position {
        'O' => Position::RoundedRock,
        '#' => Position::CubeRock,
        '.' => Position::Empty,
        _ => panic!("Unknown position {position}"),
    };
}

fn mirror_string(mirror: &Mirror) -> String {
    let mut out: String = "".to_string();
    for row in mirror {
        for position in row {
            match position {
                Position::RoundedRock => out.push('O'),
                Position::CubeRock => out.push('#'),
                Position::Empty => out.push('.'),
            }
        }
        out.push('\n');
    }
    out.push('\n');
    return out;
}

fn parse_input() -> Mirror {
    let contents = fs::read_to_string("./inputs/day14.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(parse_position).collect())
        .collect();
}

fn slide_cubes_north(mirror: &mut Mirror) {
    for col in 0..mirror[0].len() {
        for row in 0..mirror.len() {
            if mirror[row][col] == Position::Empty {
                for row2 in row..mirror.len() {
                    if mirror[row2][col] == Position::CubeRock {
                        break;
                    }
                    if mirror[row2][col] == Position::RoundedRock {
                        mirror[row][col] = Position::RoundedRock;
                        mirror[row2][col] = Position::Empty;
                        break;
                    }
                }
            }
        }
    }
}

fn slide_cubes_south(mirror: &mut Mirror) {
    for col in 0..mirror[0].len() {
        for row in (0..mirror.len()).rev() {
            if mirror[row][col] == Position::Empty {
                for row2 in (0..row).rev() {
                    if mirror[row2][col] == Position::CubeRock {
                        break;
                    }
                    if mirror[row2][col] == Position::RoundedRock {
                        mirror[row][col] = Position::RoundedRock;
                        mirror[row2][col] = Position::Empty;
                        break;
                    }
                }
            }
        }
    }
}

fn slide_cubes_west(mirror: &mut Mirror) {
    for row in 0..mirror.len() {
        for col in 0..mirror[0].len() {
            if mirror[row][col] == Position::Empty {
                for col2 in col..mirror[0].len() {
                    if mirror[row][col2] == Position::CubeRock {
                        break;
                    }
                    if mirror[row][col2] == Position::RoundedRock {
                        mirror[row][col] = Position::RoundedRock;
                        mirror[row][col2] = Position::Empty;
                        break;
                    }
                }
            }
        }
    }
}

fn slide_cubes_east(mirror: &mut Mirror) {
    for row in 0..mirror.len() {
        for col in (0..mirror[0].len()).rev() {
            if mirror[row][col] == Position::Empty {
                for col2 in (0..col).rev() {
                    if mirror[row][col2] == Position::CubeRock {
                        break;
                    }
                    if mirror[row][col2] == Position::RoundedRock {
                        mirror[row][col] = Position::RoundedRock;
                        mirror[row][col2] = Position::Empty;
                        break;
                    }
                }
            }
        }
    }
}

fn wash_cycle(mirror: &mut Mirror) {
    slide_cubes_north(mirror);
    slide_cubes_west(mirror);
    slide_cubes_south(mirror);
    slide_cubes_east(mirror);
}

fn calculate_load(mirror: &Mirror) -> u32 {
    let mirror_len = mirror.len() as u32;
    let mut total_load = 0;
    for (idx, row) in mirror.iter().enumerate() {
        total_load += row.iter().filter(|r| r == &&Position::RoundedRock).count() as u32
            * (mirror_len - idx as u32);
    }
    return total_load;
}

// Part 1

fn part1() -> u32 {
    let mut mirror = parse_input();
    slide_cubes_north(&mut mirror);
    return calculate_load(&mirror);
}

// Part 2

fn part2() -> u32 {
    let mut mirror = parse_input();
    let full_cycles = 1000000000;

    let mut seen_mirrors: HashMap<String, u32> = HashMap::new();
    let mut iter_num = 0;
    let mut start_cycle = 0;
    while iter_num < full_cycles {
        start_cycle = *seen_mirrors
            .entry(mirror_string(&mirror))
            .or_insert(iter_num);
        if start_cycle != iter_num {
            break;
        }
        wash_cycle(&mut mirror);
        iter_num += 1;
    }

    let cycles_remaining = full_cycles - start_cycle;
    for _i in 0..(cycles_remaining % (iter_num - start_cycle)) {
        wash_cycle(&mut mirror);
    }

    return calculate_load(&mirror);
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
        assert_eq!(part1(), 109345);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 112452);
    }
}
