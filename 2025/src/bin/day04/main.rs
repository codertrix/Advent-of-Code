// ---------------------------------------------------
//  Advent of Code 2025
//  Day 4: Printing Department
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day04;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------");
    println!(" Day 4: Printing Department");
    println!("----------------------------");

    day04::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
