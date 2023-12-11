use std::collections::HashSet;
use std::fs;

// Common

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entry {
    Empty,
    Galaxy,
}

type Universe = Vec<Vec<Entry>>;

#[derive(Debug)]
struct GalaxyLocation {
    number: u32,
    x: usize,
    y: usize,
}

fn parse_map(input: char) -> Entry {
    return match input {
        '#' => Entry::Galaxy,
        '.' => Entry::Empty,
        _ => panic!("Unknown symbol {input}"),
    };
}

fn parse_input() -> Universe {
    let contents = fs::read_to_string("./inputs/day11.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().map(parse_map).collect())
        .collect();
}

fn find_galaxies(universe: &Universe) -> Vec<GalaxyLocation> {
    let mut galaxies = Vec::new();
    let mut galaxy_count = 1;
    for row in 0..universe.len() {
        for col in 0..universe[row].len() {
            if universe[row][col] == Entry::Galaxy {
                galaxies.push(GalaxyLocation {
                    number: galaxy_count,
                    x: row,
                    y: col,
                });
                galaxy_count += 1;
            }
        }
    }
    return galaxies;
}

fn get_galaxy_distances(
    universe: &Universe,
    distance_fn: &dyn Fn(&GalaxyLocation, &GalaxyLocation) -> u64,
) -> u64 {
    let galaxies = find_galaxies(&universe);
    let mut seen_galaxies: HashSet<String> = HashSet::new();
    let mut distances: u64 = 0;

    for galaxy1 in galaxies.iter() {
        for galaxy2 in galaxies.iter() {
            let mut galaxy_nums = [galaxy1.number, galaxy2.number];
            galaxy_nums.sort();
            let galaxy_key: String = galaxy_nums.iter().map(|x| x.to_string() + ",").collect();
            if !seen_galaxies.contains(&galaxy_key) {
                distances += distance_fn(&galaxy1, &galaxy2);
                seen_galaxies.insert(galaxy_key);
            }
        }
    }

    return distances;
}

// Part 1

fn transpose_universe(universe: &Universe) -> Universe {
    let mut new_universe = Vec::new();
    for col in 0..universe[0].len() {
        let mut new_row = Vec::new();
        for row in 0..universe.len() {
            new_row.push(universe[row][col]);
        }
        new_universe.push(new_row);
    }
    return new_universe;
}

fn expand_universe(universe: &Universe) -> Universe {
    let mut new_universe = Vec::new();
    for row in 0..universe.len() {
        new_universe.push(universe[row].clone());
        if universe[row].iter().all(|e| *e == Entry::Empty) {
            new_universe.push(universe[row].clone());
        }
    }
    return new_universe;
}

fn galaxy_distance_1(galaxy1: &GalaxyLocation, galaxy2: &GalaxyLocation) -> u64 {
    return (galaxy2.x as i32 - galaxy1.x as i32).abs() as u64
        + (galaxy2.y as i32 - galaxy1.y as i32).abs() as u64;
}

fn part1() -> u64 {
    let universe = transpose_universe(&expand_universe(&transpose_universe(&expand_universe(
        &parse_input(),
    ))));

    return get_galaxy_distances(&universe, &galaxy_distance_1);
}

// Part 2

fn find_sparse_rows(universe: &Universe) -> HashSet<u32> {
    let mut set = HashSet::new();
    for row in 0..universe.len() {
        if universe[row].iter().all(|e| *e == Entry::Empty) {
            set.insert(row as u32);
        }
    }
    return set;
}

fn find_sparse_columns(universe: &Universe) -> HashSet<u32> {
    let mut set = HashSet::new();
    for col in 0..universe[0].len() {
        let mut is_sparse = true;
        for row in 0..universe.len() {
            if universe[row][col] == Entry::Galaxy {
                is_sparse = false;
            }
        }
        if is_sparse {
            set.insert(col as u32);
        }
    }
    return set;
}

fn part2() -> u64 {
    let universe = parse_input();
    let sparse_rows = find_sparse_rows(&universe);
    let sparse_columns = find_sparse_columns(&universe);
    let expansion = 1000000;

    let galaxy_distance_2 = |galaxy1: &GalaxyLocation, galaxy2: &GalaxyLocation| -> u64 {
        let mut x_distances = [galaxy1.x as u32, galaxy2.x as u32];
        x_distances.sort();
        let mut y_distances = [galaxy1.y as u32, galaxy2.y as u32];
        y_distances.sort();

        let mut x_total = 0;
        for x in x_distances[0]..x_distances[1] {
            if sparse_rows.contains(&x) {
                x_total += expansion;
            } else {
                x_total += 1;
            }
        }

        let mut y_total = 0;
        for y in y_distances[0]..y_distances[1] {
            if sparse_columns.contains(&y) {
                y_total += expansion;
            } else {
                y_total += 1;
            }
        }

        return x_total + y_total;
    };

    return get_galaxy_distances(&universe, &galaxy_distance_2);
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
        assert_eq!(part1(), 9545480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 406725732046);
    }
}
