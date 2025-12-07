// ---------------------------------------------------
//  Advent of Code 2025
//  Day 7: Laboratories
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day07;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------");
    println!(" Day 7: Laboratories");
    println!("---------------------");

    day07::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
