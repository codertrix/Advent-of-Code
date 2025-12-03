// ---------------------------------------------------
//  Advent of Code 2025
//  Day 3: Lobby
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day03;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------");
    println!(" Day 3: Lobby");
    println!("--------------");

    day03::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
