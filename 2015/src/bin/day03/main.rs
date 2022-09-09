// ---------------------------------------------------
//  Advent of Code 2015
//  Day 3: Perfectly Spherical Houses in a Vacuum
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day03;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------------------------");
    println!(" Day 3: Perfectly Spherical Houses in a Vacuum");
    println!("-----------------------------------------------");

    day03::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
