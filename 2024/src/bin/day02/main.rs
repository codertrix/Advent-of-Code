// ---------------------------------------------------
//  Advent of Code 2024
//  Day 2: Red-Nosed Reports
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day02;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 2: Red-Nosed Reports");
    println!("--------------------------");

    day02::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
