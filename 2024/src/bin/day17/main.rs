// ---------------------------------------------------
//  Advent of Code 2024
//  Day 17: Chronospatial Computer
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day17;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------------");
    println!(" Day 17: Chronospatial Computer");
    println!("--------------------------------");

    day17::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
