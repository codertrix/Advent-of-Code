// ---------------------------------------------------
//  Advent of Code 2022
//  Day 7: No Space Left On Device
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day07;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------------");
    println!(" Day 7: No Space Left On Device");
    println!("--------------------------------");

    day07::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
