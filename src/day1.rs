use std::fs;

fn sum_contents(contents: String, calibration_fn: &dyn Fn(&str) -> u32) -> u32 {
    let lines = contents.split("\n");

    return lines
        .filter(|line| line.len() > 0)
        .map(|line| calibration_fn(line))
        .sum();
}

fn calibration_value1(input: &str) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for c in input.chars() {
        if c.is_digit(10) {
            if first.is_none() {
                first = Some(c);
            }
            last = Some(c);
        }
    }

    let mut result = first.unwrap().to_string();
    result.push_str(&last.unwrap().to_string());

    return result.parse().unwrap();
}

fn part1() -> u32 {
    let contents = fs::read_to_string("./inputs/day1.txt").expect("No file found");
    return sum_contents(contents, &calibration_value1);
}

const NUMBER_LOOKUP: [(&str, [&str; 2]); 9] = [
    ("1", ["1", "one"]),
    ("2", ["2", "two"]),
    ("3", ["3", "three"]),
    ("4", ["4", "four"]),
    ("5", ["5", "five"]),
    ("6", ["6", "six"]),
    ("7", ["7", "seven"]),
    ("8", ["8", "eight"]),
    ("9", ["9", "nine"]),
];

fn calibration_value2(input: &str) -> u32 {
    let mut newinput = input.to_string();

    let mut first: Option<&str> = None;
    let mut last: Option<&str> = None;

    'outer: while first.is_none() && newinput.len() > 0 {
        for (num, opts) in NUMBER_LOOKUP.iter() {
            for opt in opts.iter() {
                if newinput.starts_with(opt) {
                    first = Some(num);
                    break 'outer;
                }
            }
        }
        newinput.remove(0);
    }

    'outer: while last.is_none() && newinput.len() > 0 {
        for (num, opts) in NUMBER_LOOKUP.iter() {
            for opt in opts.iter() {
                if newinput.ends_with(opt) {
                    last = Some(num);
                    break 'outer;
                }
            }
        }
        newinput.pop();
    }

    let mut result = first.unwrap().to_string();
    result.push_str(&last.unwrap().to_string());

    return result.parse().unwrap();
}

fn part2() -> u32 {
    let contents = fs::read_to_string("./inputs/day1.txt").expect("No file found");
    return sum_contents(contents, &calibration_value2);
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 55123);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 55260);
    }
}
