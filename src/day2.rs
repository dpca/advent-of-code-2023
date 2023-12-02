use std::fs;

// Common

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    number: u32,
    sets: Vec<CubeSet>,
}

fn parse_set(input: &str) -> CubeSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for entry in input.split(", ") {
        let (num, color) = entry.split_once(" ").unwrap();
        match color {
            "red" => red = num.parse().unwrap(),
            "green" => green = num.parse().unwrap(),
            "blue" => blue = num.parse().unwrap(),
            _ => panic!("Unknown color {color}"),
        }
    }

    return CubeSet { red, green, blue };
}

fn parse_game(input: &str) -> Game {
    let (game, sets) = input.split_once(": ").unwrap();
    let game_num: u32 = game.split(' ').last().unwrap().parse().unwrap();

    return Game {
        number: game_num,
        sets: sets.split("; ").map(parse_set).collect(),
    };
}

fn games() -> Vec<Game> {
    let contents = fs::read_to_string("./inputs/day2.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|line| line.len() > 0)
        .map(parse_game)
        .collect();
}

// Part 1

fn valid_set(set: &CubeSet) -> bool {
    return set.red <= 12 && set.green <= 13 && set.blue <= 14;
}

fn valid_game(game: &Game) -> bool {
    return game.sets.iter().all(valid_set);
}

fn part1() -> u32 {
    return games()
        .into_iter()
        .filter(valid_game)
        .map(|game| game.number)
        .sum();
}

// Part 2

fn get_cube_minimums(game: Game) -> CubeSet {
    let mut cube_minimums: CubeSet = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in game.sets {
        if set.red > cube_minimums.red {
            cube_minimums.red = set.red
        }
        if set.green > cube_minimums.green {
            cube_minimums.green = set.green
        }
        if set.blue > cube_minimums.blue {
            cube_minimums.blue = set.blue
        }
    }

    return cube_minimums;
}

fn cube_power(set: CubeSet) -> u32 {
    return set.red * set.green * set.blue;
}

fn part2() -> u32 {
    return games()
        .into_iter()
        .map(get_cube_minimums)
        .map(cube_power)
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
        assert_eq!(part1(), 2169);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 60948);
    }
}
