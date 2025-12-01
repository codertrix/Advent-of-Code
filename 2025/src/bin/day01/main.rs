// ---------------------------------------------------
//  Advent of Code 2025
//  Day 1: Secret Entrance
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day01;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 1: Secret Entrance");
    println!("------------------------");

    day01::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
