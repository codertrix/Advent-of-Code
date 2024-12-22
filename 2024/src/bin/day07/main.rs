// ---------------------------------------------------
//  Advent of Code 2024
//  Day 7: Bridge Repair
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day07;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 7: Bridge Repair");
    println!("----------------------");

    day07::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
