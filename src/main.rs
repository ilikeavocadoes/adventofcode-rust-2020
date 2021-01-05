use std::env;

use adventofcode_rust_2020::day13;
use adventofcode_rust_2020::day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match day.as_str() {
        "day2" => day2::run(),
        "day13" => day13::run(),
        _ => panic!("Expected a day name (e.g. day2)"),
    }
}
