// ---------------------------------------------------
//  Advent of Code 2023
//  Day 23: A Long Walk
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day23;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------");
    println!(" Day 23: A Long Walk");
    println!("---------------------");

    day23::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
