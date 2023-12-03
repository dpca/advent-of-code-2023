use std::fs;

// Common

fn parse_input() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("./inputs/day3.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();
}

const CHECK_COORDS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_symbol(char: char) -> bool {
    return !char.is_digit(10) && char != '.';
}

fn flood(map: &mut Vec<Vec<char>>, y: i32, x: i32) -> u32 {
    if y < 0 || x < 0 || y >= map.len() as i32 || x >= map[y as usize].len() as i32 {
        return 0;
    }
    if map[y as usize][x as usize].is_digit(10) {
        let mut x_start = x;
        while x_start > 0 && map[y as usize][x_start as usize - 1].is_digit(10) {
            x_start -= 1;
        }
        let mut num_str: String = String::new();
        while x_start < map[y as usize].len() as i32
            && map[y as usize][x_start as usize].is_digit(10)
        {
            num_str.push(map[y as usize][x_start as usize]);
            map[y as usize][x_start as usize] = '.';
            x_start += 1;
        }
        return num_str.parse().unwrap();
    }
    return 0;
}

// Part 1

fn part1() -> u32 {
    let mut map = parse_input();
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if is_symbol(map[y][x]) {
                for (y_diff, x_diff) in CHECK_COORDS {
                    sum += flood(&mut map, (y as i32) + y_diff, (x as i32) + x_diff);
                }
            }
        }
    }
    return sum;
}

// Part 2

fn part2() -> u32 {
    let mut map = parse_input();
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '*' {
                let mut gears: Vec<u32> = Vec::new();
                for (y_diff, x_diff) in CHECK_COORDS {
                    let part_num = flood(&mut map, (y as i32) + y_diff, (x as i32) + x_diff);
                    if part_num > 0 {
                        gears.push(part_num);
                    }
                }
                if gears.len() == 2 {
                    sum += gears.iter().product::<u32>();
                }
            }
        }
    }
    return sum;
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
        assert_eq!(part1(), 527446);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 73201705);
    }
}
