// ---------------------------------------------------
//  Advent of Code 2024
//  Day 11: Plutonian Pebbles
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day11;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------");
    println!(" Day 11: Plutonian Pebbles");
    println!("---------------------------");

    day11::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
