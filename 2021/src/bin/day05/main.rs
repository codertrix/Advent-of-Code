// ---------------------------------------------------
//  Advent of Code 2021
//  Day 5: Hydrothermal Venture
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day05;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------");
    println!(" Day 5: Hydrothermal Venture");
    println!("-----------------------------");

    day05::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
