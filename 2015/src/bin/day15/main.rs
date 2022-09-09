// ---------------------------------------------------
//  Advent of Code 2015
//  Day 15: Science for Hungry People
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day15;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------------");
    println!(" Day 15: Science for Hungry People");
    println!("-----------------------------------");

    day15::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
