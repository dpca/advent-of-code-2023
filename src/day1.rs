use std::fs;

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

fn part1(contents: String) -> u32 {
    let lines = contents.split("\n");

    let mut sum = 0;

    for line in lines {
        if line.len() > 0 {
            let num = calibration_value1(line);
            sum += num;
        }
    }

    return sum;
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

fn part2(contents: String) -> u32 {
    let lines = contents.split("\n");

    let mut sum = 0;

    for line in lines {
        if line.len() > 0 {
            let num = calibration_value2(line);
            sum += num;
        }
    }

    return sum;
}

pub fn run() {
    let contents = fs::read_to_string("./inputs/day1.txt").expect("No file found");

    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}
