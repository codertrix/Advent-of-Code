// ---------------------------------------------------
//  Advent of Code 2015
//  Day 22: Wizard Simulator 20XX
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day22;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------------");
    println!(" Day 22: Wizard Simulator 20XX");
    println!("-------------------------------");

    day22::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
