// ---------------------------------------------------
//  Advent of Code 2025
//  Day 6: Trash Compactor
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day06;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 6: Trash Compactor");
    println!("------------------------");

    day06::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
