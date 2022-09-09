// ---------------------------------------------------
//  Advent of Code 2015
//  Day 16: Aunt Sue
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day16;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------");
    println!(" Day 16: Aunt Sue");
    println!("------------------");

    day16::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
