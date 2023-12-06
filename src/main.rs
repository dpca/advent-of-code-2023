use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day as &str {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        "6" => day6::run(),
        _ => println!("Day {day} not implemented!"),
    }
}
