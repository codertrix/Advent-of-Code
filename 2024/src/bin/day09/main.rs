// ---------------------------------------------------
//  Advent of Code 2024
//  Day 9: Disk Fragmenter
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day09;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 9: Disk Fragmenter");
    println!("------------------------");

    day09::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
