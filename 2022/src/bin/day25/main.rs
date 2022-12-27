// ---------------------------------------------------
//  Advent of Code 2022
//  Day 25: Full of Hot Air
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day25;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------");
    println!(" Day 25: Full of Hot Air");
    println!("-------------------------");

    day25::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
