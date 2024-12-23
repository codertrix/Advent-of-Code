// ---------------------------------------------------
//  Advent of Code 2024
//  Day 12: Garden Groups
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day12;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------");
    println!(" Day 12: Garden Groups");
    println!("-----------------------");

    day12::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
