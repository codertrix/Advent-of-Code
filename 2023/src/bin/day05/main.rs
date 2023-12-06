// ---------------------------------------------------
//  Advent of Code 2023
//  Day 5: If You Give A Seed A Fertilizer
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day05;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------------------");
    println!(" Day 5: If You Give A Seed A Fertilizer");
    println!("----------------------------------------");

    day05::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
