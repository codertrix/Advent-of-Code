// ---------------------------------------------------
//  Advent of Code 2015
//  Day 10: Elves Look, Elves Say
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day10;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------------");
    println!(" Day 10: Elves Look, Elves Say");
    println!("-------------------------------");

    day10::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
