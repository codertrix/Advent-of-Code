// ---------------------------------------------------
//  Advent of Code 2025
//  Day 5: Cafeteria
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day05;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------");
    println!(" Day 5: Cafeteria");
    println!("------------------");

    day05::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
