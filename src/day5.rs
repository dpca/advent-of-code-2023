use std::fs::File;
use std::io::{self, BufRead};

// Common

#[derive(Debug)]
struct AlmanacMapping {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AlmanacSection {
    input: String,
    output: String,
    mappings: Vec<AlmanacMapping>,
}

fn file_reader() -> io::BufReader<File> {
    let file = File::open("./inputs/day5.txt").unwrap();
    return io::BufReader::new(file);
}

fn get_seeds(reader: &mut io::BufReader<File>) -> std::io::Result<Vec<u64>> {
    let mut seed_buffer = String::new();
    reader.read_line(&mut seed_buffer)?;
    reader.read_line(&mut seed_buffer)?;
    return Ok(seed_buffer
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect());
}

fn get_almanac_entry(reader: &mut io::BufReader<File>) -> std::io::Result<(bool, AlmanacSection)> {
    let mut section_title = String::new();
    reader.read_line(&mut section_title)?;
    let (input, output): (&str, &str) = section_title
        .split(" ")
        .next()
        .unwrap()
        .split_once("-to-")
        .unwrap();

    let mut mappings: Vec<AlmanacMapping> = Vec::new();
    let mut eof = false;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            eof = true;
            break;
        }
        if line == "\n" {
            break;
        }
        let mut nums = line.trim().split(" ").map(|n| n.parse().unwrap());
        mappings.push(AlmanacMapping {
            destination_range_start: nums.next().unwrap(),
            source_range_start: nums.next().unwrap(),
            range_length: nums.next().unwrap(),
        });
    }

    return Ok((
        eof,
        AlmanacSection {
            input: input.to_string(),
            output: output.to_string(),
            mappings,
        },
    ));
}

fn parse_input() -> (Vec<u64>, Vec<AlmanacSection>) {
    let mut almanac: Vec<AlmanacSection> = Vec::new();

    let mut reader = file_reader();
    let seeds = get_seeds(&mut reader).unwrap();
    loop {
        let (eof, section) = get_almanac_entry(&mut reader).unwrap();
        almanac.push(section);
        if eof {
            break;
        }
    }

    return (seeds, almanac);
}

// Part 1

fn map_number_to_section(number: u64, almanac: &AlmanacSection) -> u64 {
    for mapping in almanac.mappings.iter() {
        if number >= mapping.source_range_start
            && number <= mapping.source_range_start + mapping.range_length
        {
            return mapping.destination_range_start + (number - mapping.source_range_start);
        }
    }
    return number;
}

fn part1() -> u64 {
    let (seeds, almanac) = parse_input();

    let mut seed_changes = seeds.clone();
    for i in 0..seed_changes.len() {
        for section in almanac.iter() {
            seed_changes[i] = map_number_to_section(seed_changes[i], &section);
        }
    }

    return *seed_changes.iter().min().unwrap();
}

// Part 2

fn map_number_to_section_back(number: u64, almanac: &AlmanacSection) -> u64 {
    for mapping in almanac.mappings.iter() {
        if number >= mapping.destination_range_start
            && number < mapping.destination_range_start + mapping.range_length
        {
            return mapping.source_range_start + (number - mapping.destination_range_start);
        }
    }
    return number;
}

fn part2() -> u64 {
    let (seeds, mut almanac) = parse_input();
    almanac.reverse();

    let mut location_num: u64 = 0;
    loop {
        let mut this_mapped_number = location_num.clone();
        for section in almanac.iter() {
            this_mapped_number = map_number_to_section_back(this_mapped_number, &section);
        }
        for chunk in seeds.chunks(2) {
            let seed_start = chunk[0];
            let seed_end = chunk[0] + chunk[1];
            if this_mapped_number >= seed_start && this_mapped_number <= seed_end {
                return location_num;
            }
        }
        location_num += 1;
    }
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
        assert_eq!(part1(), 174137457);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1493866);
    }
}
