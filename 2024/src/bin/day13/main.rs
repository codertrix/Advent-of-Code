// ---------------------------------------------------
//  Advent of Code 2024
//  Day 13: Claw Contraption
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day13;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 13: Claw Contraption");
    println!("--------------------------");

    day13::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
