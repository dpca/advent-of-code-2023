use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day as &str {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        _ => println!("Day {day} not implemented!"),
    }
}
