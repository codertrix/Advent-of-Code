// ---------------------------------------------------
//  Advent of Code 2024
//  Day 19: Linen Layout
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day19;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 19: Linen Layout");
    println!("----------------------");

    day19::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
