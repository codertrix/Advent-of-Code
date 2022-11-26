// ---------------------------------------------------
//  Advent of Code 2015
//  Day 13: Knights of the Dinner Table
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day13;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------------------");
    println!(" Day 13: Knights of the Dinner Table");
    println!("-------------------------------------");

    day13::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
