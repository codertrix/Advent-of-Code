// ---------------------------------------------------
//  Advent of Code 2024
//  Day 25: Code Chronicle
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day25;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 25: Code Chronicle");
    println!("------------------------");

    day25::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
