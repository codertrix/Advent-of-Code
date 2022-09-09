// ---------------------------------------------------
//  Advent of Code 2015
//  Day 24: It Hangs in the Balance
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day24;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------------");
    println!(" Day 24: It Hangs in the Balance");
    println!("---------------------------------");

    day24::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
