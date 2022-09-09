// ---------------------------------------------------
//  Advent of Code 2015
//  Day 4: The Ideal Stocking Stuffer
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day04;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------------");
    println!(" Day 4: The Ideal Stocking Stuffer");
    println!("-----------------------------------");

    day04::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
