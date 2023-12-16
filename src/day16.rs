use std::collections::HashSet;
use std::fs;

// Common

#[derive(Debug, PartialEq, Clone, Copy)]
enum PositionType {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Position {
    position_type: PositionType,
    energized: bool,
    beams: HashSet<Direction>,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            position_type: self.position_type,
            energized: self.energized,
            beams: self.beams.clone(),
        }
    }
}

fn parse_position(position: char) -> Position {
    let position_type = match position {
        '.' => PositionType::Empty,
        '/' => PositionType::MirrorUp,
        '\\' => PositionType::MirrorDown,
        '|' => PositionType::SplitterVertical,
        '-' => PositionType::SplitterHorizontal,
        _ => panic!("Unknown position {position}"),
    };

    return Position {
        position_type,
        energized: false,
        beams: HashSet::new(),
    };
}

type Contraption = Vec<Vec<Position>>;

fn run_beam(contraption: &mut Contraption, row: i32, col: i32, direction: Direction) {
    if row < 0 || col < 0 || row >= contraption.len() as i32 || col >= contraption[0].len() as i32 {
        return;
    }

    let position = &mut contraption[row as usize][col as usize];
    if position.beams.contains(&direction) {
        return;
    }

    position.energized = true;
    position.beams.insert(direction);

    match direction {
        Direction::North => match position.position_type {
            PositionType::Empty | PositionType::SplitterVertical => {
                run_beam(contraption, row + 1, col, direction);
            }
            PositionType::MirrorDown => {
                run_beam(contraption, row, col + 1, Direction::West);
            }
            PositionType::MirrorUp => {
                run_beam(contraption, row, col - 1, Direction::East);
            }
            PositionType::SplitterHorizontal => {
                run_beam(contraption, row, col + 1, Direction::West);
                run_beam(contraption, row, col - 1, Direction::East);
            }
        },
        Direction::South => match position.position_type {
            PositionType::Empty | PositionType::SplitterVertical => {
                run_beam(contraption, row - 1, col, direction);
            }
            PositionType::MirrorDown => {
                run_beam(contraption, row, col - 1, Direction::East);
            }
            PositionType::MirrorUp => {
                run_beam(contraption, row, col + 1, Direction::West);
            }
            PositionType::SplitterHorizontal => {
                run_beam(contraption, row, col + 1, Direction::West);
                run_beam(contraption, row, col - 1, Direction::East);
            }
        },
        Direction::East => match position.position_type {
            PositionType::Empty | PositionType::SplitterHorizontal => {
                run_beam(contraption, row, col - 1, direction);
            }
            PositionType::MirrorDown => {
                run_beam(contraption, row - 1, col, Direction::South);
            }
            PositionType::MirrorUp => {
                run_beam(contraption, row + 1, col, Direction::North);
            }
            PositionType::SplitterVertical => {
                run_beam(contraption, row - 1, col, Direction::South);
                run_beam(contraption, row + 1, col, Direction::North);
            }
        },
        Direction::West => match position.position_type {
            PositionType::Empty | PositionType::SplitterHorizontal => {
                run_beam(contraption, row, col + 1, direction);
            }
            PositionType::MirrorUp => {
                run_beam(contraption, row - 1, col, Direction::South);
            }
            PositionType::MirrorDown => {
                run_beam(contraption, row + 1, col, Direction::North);
            }
            PositionType::SplitterVertical => {
                run_beam(contraption, row - 1, col, Direction::South);
                run_beam(contraption, row + 1, col, Direction::North);
            }
        },
    }
}

fn parse_input() -> Contraption {
    let contents = fs::read_to_string("./inputs/day16.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(parse_position).collect())
        .collect();
}

fn energized(contraption: &Contraption, row: i32, col: i32, direction: Direction) -> u32 {
    let mut new_contraption = contraption.clone();
    run_beam(&mut new_contraption, row, col, direction);
    return new_contraption
        .iter()
        .map(|l| l.iter().filter(|p| p.energized).count() as u32)
        .sum();
}

// Part 1

fn part1() -> u32 {
    let contraption = parse_input();
    return energized(&contraption, 0, 0, Direction::West);
}

// Part 2

fn part2() -> u32 {
    let contraption = parse_input();

    let mut max_energized = 0;
    for row in 0..contraption.len() {
        let energy_west = energized(&contraption, row as i32, 0, Direction::West);
        if energy_west > max_energized {
            max_energized = energy_west;
        }

        let energy_east = energized(
            &contraption,
            row as i32,
            contraption[0].len() as i32,
            Direction::East,
        );
        if energy_east > max_energized {
            max_energized = energy_east;
        }
    }

    for col in 0..contraption[0].len() {
        let energy_north = energized(&contraption, 0, col as i32, Direction::North);
        if energy_north > max_energized {
            max_energized = energy_north;
        }

        let energy_south = energized(
            &contraption,
            contraption.len() as i32,
            col as i32,
            Direction::South,
        );
        if energy_south > max_energized {
            max_energized = energy_south;
        }
    }

    return max_energized;
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
        assert_eq!(part1(), 7939);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 8318);
    }
}
