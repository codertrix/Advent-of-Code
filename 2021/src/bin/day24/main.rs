// ---------------------------------------------------
//  Advent of Code 2021
//  Day 24: Arithmetic Logic Unit
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day24;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------------");
    println!(" Day 24: Arithmetic Logic Unit");
    println!("-------------------------------");

    day24::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
