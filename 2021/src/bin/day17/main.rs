// ---------------------------------------------------
//  Advent of Code 2021
//  Day 17: Trick Shot
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day17;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------");
    println!(" Day 17: Trick Shot");
    println!("--------------------");

    day17::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
