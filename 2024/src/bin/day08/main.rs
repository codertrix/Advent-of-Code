// ---------------------------------------------------
//  Advent of Code 2024
//  Day 8: Resonant Collinearity
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day08;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 8: Resonant Collinearity");
    println!("------------------------------");

    day08::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
