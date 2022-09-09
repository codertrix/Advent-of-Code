// ---------------------------------------------------
//  Advent of Code 2015
//  Day 14: Reindeer Olympics
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day14;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------");
    println!(" Day 14: Reindeer Olympics");
    println!("---------------------------");

    day14::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
