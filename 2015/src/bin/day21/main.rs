// ---------------------------------------------------
//  Advent of Code 2015
//  Day 21: RPG Simulator 20XX
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day21;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------");
    println!(" Day 21: RPG Simulator 20XX");
    println!("----------------------------");

    day21::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
