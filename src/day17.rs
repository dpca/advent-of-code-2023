use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

// Common

type CityMap = Vec<Vec<u32>>;

fn parse_input() -> CityMap {
    let contents = fs::read_to_string("./inputs/day17.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node {
    row: usize,
    col: usize,
    direction: Direction,
    straights: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn overwrite_print_position(map_str: &mut Vec<Vec<String>>, position: &Node) {
    match position.direction {
        Direction::North => map_str[position.row][position.col] = "\x1b[1;31m^\x1b[0m".to_string(),
        Direction::South => map_str[position.row][position.col] = "\x1b[1;31mv\x1b[0m".to_string(),
        Direction::East => map_str[position.row][position.col] = "\x1b[1;31m>\x1b[0m".to_string(),
        Direction::West => map_str[position.row][position.col] = "\x1b[1;31m<\x1b[0m".to_string(),
    }
}

#[allow(dead_code)]
fn print_map(map: &CityMap, previous: &HashMap<Node, Node>, last: &Node) {
    let mut map_str: Vec<Vec<String>> = map
        .iter()
        .map(|r| r.iter().map(|c| c.to_string()).collect())
        .collect();
    let mut position = last.clone();

    overwrite_print_position(&mut map_str, &position);
    while let Some(last_position) = previous.get(&position) {
        overwrite_print_position(&mut map_str, last_position);
        position = last_position.clone();
    }

    for row in map_str {
        for pos in row {
            print!("{}", pos);
        }
        print!("\n");
    }
    print!("\n");
}

fn add_neighbor(
    neighbors: &mut Vec<Node>,
    neighbor_filter: &dyn Fn(&Node, &Direction) -> bool,
    position: Node,
    direction: Direction,
    row_max: usize,
    col_max: usize,
) {
    if neighbor_filter(&position, &direction) {
        return;
    }

    let mut row = position.row;
    let mut col = position.col;

    match direction {
        Direction::North => {
            if position.row > 0 {
                row -= 1
            } else {
                return;
            }
        }
        Direction::South => {
            if position.row < row_max {
                row += 1
            } else {
                return;
            }
        }
        Direction::East => {
            if position.col < col_max {
                col += 1
            } else {
                return;
            }
        }
        Direction::West => {
            if position.col > 0 {
                col -= 1
            } else {
                return;
            }
        }
    }

    let straights = if position.direction == direction {
        position.straights + 1
    } else {
        0
    };

    neighbors.push(Node {
        row,
        col,
        direction,
        straights,
    });
}

fn get_lowest_cost(map: &CityMap, neighbor_filter: &dyn Fn(&Node, &Direction) -> bool) -> u32 {
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;

    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<Node, u32> = HashMap::new();
    //let mut previous: HashMap<Node, Node> = HashMap::new();

    dist.insert(
        Node {
            row: 0,
            col: 1,
            direction: Direction::East,
            straights: 0,
        },
        map[0][1],
    );
    heap.push(State {
        position: Node {
            row: 0,
            col: 1,
            direction: Direction::East,
            straights: 0,
        },
        cost: map[0][1],
    });
    dist.insert(
        Node {
            row: 1,
            col: 0,
            direction: Direction::South,
            straights: 0,
        },
        map[1][0],
    );
    heap.push(State {
        position: Node {
            row: 1,
            col: 0,
            direction: Direction::South,
            straights: 0,
        },
        cost: map[1][0],
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position.row == row_max && position.col == col_max {
            //print_map(&map, &previous, &position);
            return cost;
        }

        if cost > dist[&position] {
            continue;
        }

        let mut neighbors: Vec<Node> = Vec::new();
        match position.direction {
            Direction::North => {
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::East,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::West,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::North,
                    row_max,
                    col_max,
                );
            }
            Direction::South => {
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::East,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::West,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::South,
                    row_max,
                    col_max,
                );
            }
            Direction::East => {
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::North,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::South,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::East,
                    row_max,
                    col_max,
                );
            }
            Direction::West => {
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::North,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::South,
                    row_max,
                    col_max,
                );
                add_neighbor(
                    &mut neighbors,
                    neighbor_filter,
                    position,
                    Direction::West,
                    row_max,
                    col_max,
                );
            }
        }

        for neighbor in neighbors {
            let next = State {
                cost: cost + map[neighbor.row][neighbor.col],
                position: neighbor,
            };

            if !dist.contains_key(&neighbor) || next.cost < dist[&neighbor] {
                heap.push(next);
                dist.insert(neighbor, next.cost);
                //previous.insert(neighbor, position);
            }
        }
    }

    return 1;
}

// Part 1

const MAX_STRAIGHTS_1: u8 = 2;

fn neighbor_filter_1(position: &Node, direction: &Direction) -> bool {
    return &position.direction == direction && position.straights == MAX_STRAIGHTS_1;
}

fn part1() -> u32 {
    let map = parse_input();
    return get_lowest_cost(&map, &neighbor_filter_1);
}

// Part 2

const MIN_STRAIGHTS_2: u8 = 3;
const MAX_STRAIGHTS_2: u8 = 9;

fn neighbor_filter_2(position: &Node, direction: &Direction) -> bool {
    return (&position.direction == direction && position.straights == MAX_STRAIGHTS_2)
        || (&position.direction != direction && position.straights < MIN_STRAIGHTS_2);
}

fn part2() -> u32 {
    let map = parse_input();
    return get_lowest_cost(&map, &neighbor_filter_2);
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
        assert_eq!(part1(), 967);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1101);
    }
}
