// ---------------------------------------------------
//  Advent of Code 2022
//  Day 12: Hill Climbing Algorithm
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day12;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------------");
    println!(" Day 12: Hill Climbing Algorithm");
    println!("---------------------------------");

    day12::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
