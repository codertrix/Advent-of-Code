// ---------------------------------------------------
//  Advent of Code 2023
//  Day 8: Haunted Wasteland
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day08;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 8: Haunted Wasteland");
    println!("--------------------------");

    day08::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
