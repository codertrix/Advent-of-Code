// ---------------------------------------------------
//  Advent of Code 2024
//  Day 4: Ceres Search
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day04;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------");
    println!(" Day 4: Ceres Search");
    println!("---------------------");

    day04::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
