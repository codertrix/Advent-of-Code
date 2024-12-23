// ---------------------------------------------------
//  Advent of Code 2024
//  Day 10: Hoof It
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day10;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------");
    println!(" Day 10: Hoof It");
    println!("-----------------");

    day10::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
