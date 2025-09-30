// ---------------------------------------------------
//  Advent of Code 2024
//  Day 18: RAM Run
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day18;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------");
    println!(" Day 18: RAM Run");
    println!("-----------------");

    day18::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
