use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day as &str {
        "1" => day1::run(),
        _ => println!("Day {day} not implemented!"),
    }
}
