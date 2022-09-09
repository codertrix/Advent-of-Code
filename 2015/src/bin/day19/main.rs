// ---------------------------------------------------
//  Advent of Code 2015
//  Day 19: Medicine for Rudolph
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day19;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 19: Medicine for Rudolph");
    println!("------------------------------");

    day19::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
