// ---------------------------------------------------
//  Advent of Code 2015
//  Day 17: No Such Thing as Too Much
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day17;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------------");
    println!(" Day 17: No Such Thing as Too Much");
    println!("-----------------------------------");

    day17::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
