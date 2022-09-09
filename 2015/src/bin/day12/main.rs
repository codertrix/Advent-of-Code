// ---------------------------------------------------
//  Advent of Code 2015
//  Day 12: JSAbacusFramework.io
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day12;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 12: JSAbacusFramework.io");
    println!("------------------------------");

    day12::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
