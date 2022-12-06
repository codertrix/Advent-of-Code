// ---------------------------------------------------
//  Advent of Code 2022
//  Day 6: Tuning Trouble
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day06;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------");
    println!(" Day 6: Tuning Trouble");
    println!("-----------------------");

    day06::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
