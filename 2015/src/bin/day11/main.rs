// ---------------------------------------------------
//  Advent of Code 2015
//  Day 11: Corporate Policy
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day11;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 11: Corporate Policy");
    println!("--------------------------");

    day11::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
