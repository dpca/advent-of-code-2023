use std::fs;

// Common

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Empty,
    Visited,
    AnimalStart,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_pipe(input: char) -> Pipe {
    return match input {
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::NorthAndEast,
        'J' => Pipe::NorthAndWest,
        '7' => Pipe::SouthAndWest,
        'F' => Pipe::SouthAndEast,
        '.' => Pipe::Ground,
        'S' => Pipe::AnimalStart,
        _ => panic!("Pipe {input} not found!"),
    };
}

fn print_map(map: &Vec<Vec<Pipe>>) {
    let mut out: String = "".to_string();
    for line in map {
        for point in line {
            match point {
                Pipe::Vertical => out.push('│'),
                Pipe::Horizontal => out.push('─'),
                Pipe::NorthAndEast => out.push('└'),
                Pipe::NorthAndWest => out.push('┘'),
                Pipe::SouthAndWest => out.push('┐'),
                Pipe::SouthAndEast => out.push('┌'),
                Pipe::Ground => out.push('.'),
                Pipe::Empty => out.push(' '),
                Pipe::Visited => out.push(' '),
                Pipe::AnimalStart => out.push('O'),
            }
        }
        out.push('\n');
    }
    print!("{}", out);
}

fn next_position(
    pipe: Pipe,
    (x, y, direction): (usize, usize, Direction),
) -> (bool, usize, usize, Direction) {
    return match pipe {
        Pipe::Vertical => {
            if direction == Direction::North {
                (false, x, y + 1, Direction::North)
            } else {
                (false, x, y - 1, Direction::South)
            }
        }
        Pipe::Horizontal => {
            if direction == Direction::West {
                (false, x + 1, y, Direction::West)
            } else {
                (false, x - 1, y, Direction::East)
            }
        }
        Pipe::NorthAndEast => {
            if direction == Direction::North {
                (false, x + 1, y, Direction::West)
            } else {
                (false, x, y - 1, Direction::South)
            }
        }
        Pipe::NorthAndWest => {
            if direction == Direction::North {
                (false, x - 1, y, Direction::East)
            } else {
                (false, x, y - 1, Direction::South)
            }
        }
        Pipe::SouthAndWest => {
            if direction == Direction::South {
                (false, x - 1, y, Direction::East)
            } else {
                (false, x, y + 1, Direction::North)
            }
        }
        Pipe::SouthAndEast => {
            if direction == Direction::South {
                (false, x + 1, y, Direction::West)
            } else {
                (false, x, y + 1, Direction::North)
            }
        }
        Pipe::AnimalStart => (true, x, y, direction),
        Pipe::Ground | Pipe::Empty | Pipe::Visited => panic!("Should never be on ground"),
    };
}

fn parse_input() -> Vec<Vec<Pipe>> {
    let contents = fs::read_to_string("./inputs/day10.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().map(parse_pipe).collect())
        .collect();
}

fn get_animal_start(map: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Pipe::AnimalStart {
                return (x, y);
            }
        }
    }
    panic!("Couldn't find animal start");
}

fn get_first_pipe(map: &Vec<Vec<Pipe>>, (x, y): (usize, usize)) -> (usize, usize, Direction) {
    let mut pipe = map[y][x + 1];
    if pipe == Pipe::Horizontal || pipe == Pipe::NorthAndWest || pipe == Pipe::SouthAndWest {
        return (x + 1, y, Direction::West);
    }

    pipe = map[y][x - 1];
    if pipe == Pipe::Horizontal || pipe == Pipe::NorthAndEast || pipe == Pipe::SouthAndEast {
        return (x - 1, y, Direction::East);
    }

    pipe = map[y + 1][x];
    if pipe == Pipe::Vertical || pipe == Pipe::NorthAndEast || pipe == Pipe::NorthAndWest {
        return (x, y + 1, Direction::North);
    }

    if pipe == Pipe::Horizontal || pipe == Pipe::SouthAndEast || pipe == Pipe::SouthAndWest {
        return (x, y - 1, Direction::North);
    }

    panic!("Couldn't find first pipe");
}

// Part 1

fn part1() -> u32 {
    let pipe_map = parse_input();

    let start = get_animal_start(&pipe_map);
    let (mut x, mut y, mut direction) = get_first_pipe(&pipe_map, start);
    let mut done;
    let mut counter = 1;
    loop {
        let pipe = pipe_map[y][x];
        (done, x, y, direction) = next_position(pipe, (x, y, direction));
        if done {
            break;
        }
        counter += 1;
    }

    return counter / 2;
}

// Part 2

fn get_animal_start_and_new_map(map: &Vec<Vec<Pipe>>) -> (usize, usize, Vec<Vec<Pipe>>) {
    let mut animal_start = (0, 0);
    let mut new_map = Vec::new();

    for y in 0..map.len() {
        let mut new_row = Vec::new();
        let mut empty_row = Vec::new();
        for x in 0..map[y].len() {
            if map[y][x] == Pipe::AnimalStart {
                animal_start = (x, y);
            }
            new_row.push(Pipe::Ground);
            new_row.push(Pipe::Empty);
            empty_row.push(Pipe::Empty);
            empty_row.push(Pipe::Empty);
        }
        new_map.push(new_row);
        new_map.push(empty_row);
    }

    return (animal_start.0, animal_start.1, new_map);
}

fn flood_fill(map: &mut Vec<Vec<Pipe>>, x: usize, y: usize) {
    if map[y][x] == Pipe::Empty || map[y][x] == Pipe::Ground {
        map[y][x] = Pipe::Visited;
        if y < map.len() - 1 {
            flood_fill(map, x, y + 1);
        }
        if x < map[y].len() - 1 {
            flood_fill(map, x + 1, y);
        }
        if y > 0 {
            flood_fill(map, x, y - 1);
        }
        if x > 0 {
            flood_fill(map, x - 1, y);
        }
    }
}

fn backfill_pipe(sparse_map: &mut Vec<Vec<Pipe>>, x: usize, y: usize, direction: Direction) {
    match direction {
        Direction::East => sparse_map[y * 2][x * 2 + 1] = Pipe::Horizontal,
        Direction::West => sparse_map[y * 2][x * 2 - 1] = Pipe::Horizontal,
        Direction::North => sparse_map[y * 2 - 1][x * 2] = Pipe::Vertical,
        Direction::South => sparse_map[y * 2 + 1][x * 2] = Pipe::Vertical,
    }
}

fn part2() -> usize {
    let pipe_map = parse_input();

    let (start_x, start_y, mut sparse_map) = get_animal_start_and_new_map(&pipe_map);
    sparse_map[start_y][start_x] = Pipe::AnimalStart;

    let (mut x, mut y, mut direction) = get_first_pipe(&pipe_map, (start_x, start_y));
    backfill_pipe(&mut sparse_map, x, y, direction);

    let mut done;
    loop {
        let pipe = pipe_map[y][x];
        sparse_map[y * 2][x * 2] = pipe;
        (done, x, y, direction) = next_position(pipe, (x, y, direction));
        backfill_pipe(&mut sparse_map, x, y, direction);
        if done {
            break;
        }
    }

    flood_fill(&mut sparse_map, 0, 0);
    print_map(&sparse_map);

    return sparse_map
        .iter()
        .flatten()
        .filter(|p| **p == Pipe::Ground)
        .count();
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
        assert_eq!(part1(), 6773);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 966);
    }
}
