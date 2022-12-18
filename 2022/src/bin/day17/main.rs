// ---------------------------------------------------
//  Advent of Code 2022
//  Day 17: Pyroclastic Flow
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day17;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 17: Pyroclastic Flow");
    println!("--------------------------");

    day17::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
