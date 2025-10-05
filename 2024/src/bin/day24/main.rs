// ---------------------------------------------------
//  Advent of Code 2024
//  Day 24: Crossed Wires
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day24;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------");
    println!(" Day 24: Crossed Wires");
    println!("-----------------------");

    day24::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
