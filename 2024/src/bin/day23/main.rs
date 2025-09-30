// ---------------------------------------------------
//  Advent of Code 2024
//  Day 23: LAN Party
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day23;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------");
    println!(" Day 23: LAN Party");
    println!("-------------------");

    day23::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
