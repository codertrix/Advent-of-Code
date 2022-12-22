// ---------------------------------------------------
//  Advent of Code 2022
//  Day 20: Grove Positioning System
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day20;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------------");
    println!(" Day 20: Grove Positioning System");
    println!("----------------------------------");

    day20::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
