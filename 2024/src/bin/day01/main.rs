// ---------------------------------------------------
//  Advent of Code 2024
//  Day 1: Historian Hysteria
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day01;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------");
    println!(" Day 1: Historian Hysteria");
    println!("---------------------------");

    day01::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
