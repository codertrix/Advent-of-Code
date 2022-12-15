// ---------------------------------------------------
//  Advent of Code 2022
//  Day 15: Beacon Exclusion Zone
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day15;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------------");
    println!(" Day 15: Beacon Exclusion Zone");
    println!("-------------------------------");

    day15::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
