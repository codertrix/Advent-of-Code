// ---------------------------------------------------
//  Advent of Code 2021
//  Day 2: Dive!
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day02;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------");
    println!(" Day 2: Dive!");
    println!("--------------");

    day02::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
