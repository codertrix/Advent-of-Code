// ---------------------------------------------------
//  Advent of Code 2023
//  Day 21: Step Counter
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day21;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 21: Step Counter");
    println!("----------------------");

    day21::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
