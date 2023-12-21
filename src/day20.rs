use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

struct PulseRecord {
    pulse_type: Pulse,
    input: String,
    output: String,
}

fn new_pulse_records(
    input: &str,
    destinations: &Vec<String>,
    pulse_type: Pulse,
) -> Vec<PulseRecord> {
    destinations
        .iter()
        .map(|n| PulseRecord {
            pulse_type,
            input: input.to_string(),
            output: n.to_string(),
        })
        .collect()
}

trait Pulsable {
    fn pulse(&mut self, input: &String, pulse_type: &Pulse) -> Vec<PulseRecord>;
    fn print(&self);
    fn add_input(&mut self, input: String);
    fn get_name(&self) -> String;
    fn get_destinations(&self) -> Vec<String>;
}

#[derive(Debug)]
struct FlipFlopModule {
    name: String,
    on: bool,
    destinations: Vec<String>,
}

impl Pulsable for FlipFlopModule {
    fn pulse(&mut self, _input: &String, pulse_type: &Pulse) -> Vec<PulseRecord> {
        if *pulse_type == Pulse::Low {
            self.on = !self.on;
            let new_pulse_type = if self.on { Pulse::High } else { Pulse::Low };
            return new_pulse_records(&self.name, &self.destinations, new_pulse_type);
        }
        return Vec::new();
    }

    fn print(&self) {
        println!(
            "FlipFlopModule {} - out: {}",
            self.name,
            self.destinations.join(", ")
        );
    }

    fn add_input(&mut self, _input: String) {}

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_destinations(&self) -> Vec<String> {
        self.destinations.iter().map(|d| d.to_string()).collect()
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    name: String,
    inputs: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Pulsable for ConjunctionModule {
    fn pulse(&mut self, input: &String, pulse_type: &Pulse) -> Vec<PulseRecord> {
        self.inputs.insert(input.to_string(), *pulse_type);
        let new_pulse_type = if self.inputs.values().all(|i| *i == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };
        return new_pulse_records(&self.name, &self.destinations, new_pulse_type);
    }

    fn print(&self) {
        println!(
            "ConjunctionModule {} - in: {} - out: {}",
            self.name,
            self.inputs
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.destinations.join(", ")
        );
    }

    fn add_input(&mut self, input: String) {
        self.inputs.insert(input, Pulse::Low);
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_destinations(&self) -> Vec<String> {
        self.destinations.iter().map(|d| d.to_string()).collect()
    }
}

#[derive(Debug)]
struct BroadcastModule {
    destinations: Vec<String>,
}

impl Pulsable for BroadcastModule {
    fn pulse(&mut self, _input: &String, _pulse_type: &Pulse) -> Vec<PulseRecord> {
        return new_pulse_records("broadcaster", &self.destinations, Pulse::Low);
    }

    fn print(&self) {
        println!("BroadcastModule");
    }

    fn add_input(&mut self, _input: String) {}

    fn get_name(&self) -> String {
        "broadcaster".to_string()
    }

    fn get_destinations(&self) -> Vec<String> {
        self.destinations.iter().map(|d| d.to_string()).collect()
    }
}

fn get_name_and_destinations(chars: &mut Chars) -> (String, Vec<String>) {
    let name: String = chars.by_ref().take_while(|&c| c != ' ').collect();
    chars.next();
    chars.next();
    chars.next();
    let destinations: Vec<String> = chars
        .take_while(|_| true)
        .collect::<String>()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    return (name, destinations);
}

fn parse_line(input: &str) -> Box<dyn Pulsable> {
    let mut chars = input.chars();
    let first_char = chars.next();

    let (name, destinations) = get_name_and_destinations(&mut chars);

    match first_char {
        Some('%') => Box::new(FlipFlopModule {
            name,
            on: false,
            destinations,
        }),
        Some('&') => Box::new(ConjunctionModule {
            name,
            inputs: HashMap::new(),
            destinations,
        }),
        Some('b') => Box::new(BroadcastModule { destinations }),
        _ => panic!("Unknown module"),
    }
}

fn parse_input() -> Vec<Box<dyn Pulsable>> {
    let contents = fs::read_to_string("./inputs/day20.txt").expect("File not found");

    return contents
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(parse_line)
        .collect();
}

// Part 1

fn part1() -> u32 {
    let input = parse_input();
    let mut modules_by_name = HashMap::new();
    for module in input {
        modules_by_name.insert(module.get_name(), module);
    }

    for input_name in modules_by_name
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>()
    {
        let input = modules_by_name.get(&input_name).unwrap();
        for output in input.get_destinations().iter() {
            match modules_by_name.get_mut(output) {
                Some(input) => input.add_input(input_name.to_string()),
                None => (),
            }
        }
    }

    let mut low_pulses: Vec<PulseRecord> = Vec::new();
    let mut high_pulses: Vec<PulseRecord> = Vec::new();
    let mut queue: VecDeque<PulseRecord> = VecDeque::new();

    for _i in 0..1000 {
        queue.push_back(PulseRecord {
            input: "button".to_string(),
            output: "broadcaster".to_string(),
            pulse_type: Pulse::Low,
        });

        while let Some(pulse) = queue.pop_front() {
            match modules_by_name.get_mut(&pulse.output) {
                Some(module) => {
                    for new_pulse in module.pulse(&pulse.input, &pulse.pulse_type) {
                        queue.push_back(new_pulse);
                    }
                }
                None => (),
            }
            if pulse.pulse_type == Pulse::Low {
                low_pulses.push(pulse);
            } else {
                high_pulses.push(pulse);
            }
        }
    }

    return low_pulses.len() as u32 * high_pulses.len() as u32;
}

// Part 2

fn part2() -> u32 {
    return 1;
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
        assert_eq!(part1(), 980457412);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1);
    }
}
